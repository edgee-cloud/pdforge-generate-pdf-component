mod helpers;
mod pdforge_payload;
use bindings::wasi::http::types::{IncomingRequest, ResponseOutparam};
use helpers::body::{Json, RawJson};
use pdforge_payload::PdforgeGeneratePdfPayload;
use std::collections::HashMap;

mod bindings {
    wit_bindgen::generate!({
        path: ".edgee/wit",
        world: "edge-function",
        generate_all,
        pub_export_macro: true,
        default_bindings_module: "$crate::bindings",
    });
}

struct Component;
bindings::export!(Component);

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(req: IncomingRequest, resp: ResponseOutparam) {
        helpers::run(req, resp, Self::handle_json_request);
    }
}

impl Component {
    fn handle_json_request(
        req: http::Request<Json<serde_json::Value>>,
    ) -> Result<http::Response<RawJson<String>>, anyhow::Error> {
        let settings = Settings::from_req(&req)?;

        let Json(body) = req.body();

        let payload = PdforgeGeneratePdfPayload::new(body, &settings.template_id);

        let pdforge_response = payload.send(&settings.api_key)?;

        let status_code = pdforge_response.status_code();
        let response_body =
            String::from_utf8_lossy(&pdforge_response.body().unwrap_or_default()).to_string();

        // note: Content-type is already set by helpers::run_json
        Ok(http::Response::builder()
            .status(status_code)
            .body(RawJson(response_body))?)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Settings {
    pub api_key: String,
    pub template_id: String,
}

impl Settings {
    pub fn new(headers: &http::header::HeaderMap) -> anyhow::Result<Self> {
        let value = headers
            .get("x-edgee-component-settings")
            .ok_or_else(|| anyhow::anyhow!("Missing 'x-edgee-component-settings' header"))
            .and_then(|value| value.to_str().map_err(Into::into))?;
        let data: HashMap<String, String> = serde_json::from_str(value)?;

        Ok(Self {
            api_key: data
                .get("api_key")
                .ok_or_else(|| anyhow::anyhow!("Missing api_key setting"))?
                .to_string(),
            template_id: data
                .get("template_id")
                .ok_or_else(|| anyhow::anyhow!("Missing template_id setting"))?
                .to_string(),
        })
    }

    pub fn from_req<B>(req: &http::Request<B>) -> anyhow::Result<Self> {
        Self::new(req.headers())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::{HeaderValue, Request};
    use lazy_static;
    use serde_json::json;
    use std::sync::Mutex;

    lazy_static::lazy_static! {
        static ref SEND_CALLED: Mutex<bool> = Mutex::new(false);
    }

    // Mock send method to avoid real HTTP call
    pub struct MockResponse;
    impl MockResponse {
        pub fn status_code(&self) -> u16 {
            200
        }
        pub fn body(&self) -> Option<Vec<u8>> {
            Some(
                json!({"signedUrl": "https://example.com/signed-url"})
                    .to_string()
                    .into_bytes(),
            )
        }
    }

    impl PdforgeGeneratePdfPayload {
        pub fn send(&self, _api_key: &str) -> anyhow::Result<MockResponse> {
            *SEND_CALLED.lock().unwrap() = true;
            Ok(MockResponse)
        }
    }

    #[test]
    fn test_settings_new() {
        let mut headers = http::header::HeaderMap::new();
        headers.insert(
            "x-edgee-component-settings",
            HeaderValue::from_static(
                r#"{"api_key": "test_value", "template_id": "test_template_id"}"#,
            ),
        );

        let settings = Settings::new(&headers).unwrap();
        assert_eq!(settings.api_key, "test_value");
        assert_eq!(settings.template_id, "test_template_id");
    }

    #[test]
    fn test_settings_new_missing_header() {
        let headers = http::header::HeaderMap::new();
        let result = Settings::new(&headers);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Missing 'x-edgee-component-settings' header"
        );
    }

    #[test]
    fn test_settings_new_invalid_json() {
        let mut headers = http::header::HeaderMap::new();
        headers.insert(
            "x-edgee-component-settings",
            HeaderValue::from_static(r#"not a json"#),
        );
        let result = Settings::new(&headers);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_json_request_success() {
        // Prepare request with headers and body
        let body = json!({
            "username": "JohnyDoe",
            "email": "John@Doe.com",
        });
        let req = Request::builder()
            .header(
                "x-edgee-component-settings",
                r#"{"api_key": "test_value", "template_id": "test_template_id"}"#,
            )
            .body(Json(body))
            .unwrap();

        // Call the handler
        let result = Component::handle_json_request(req);

        // Assert
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert_eq!(resp.status(), 200);
        let RawJson(response_body) = resp.body();
        assert_eq!(
            response_body.to_string(),
            r#"{"signedUrl":"https://example.com/signed-url"}"#
        );
        assert!(*SEND_CALLED.lock().unwrap());
    }
}
