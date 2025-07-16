mod helpers;
mod pdforge_payload;
mod world;

use std::collections::HashMap;

use pdforge_payload::PdforgeGeneratePdfPayload;
use world::bindings::exports::wasi::http::incoming_handler::Guest;
use world::bindings::wasi::http::types::IncomingRequest;
use world::bindings::wasi::http::types::Method;
use world::bindings::wasi::http::types::ResponseOutparam;
use world::bindings::Component;

impl Guest for Component {
    fn handle(req: IncomingRequest, resp: ResponseOutparam) {
        let settings = match Settings::from_req(&req) {
            Ok(settings) => settings,
            Err(_) => {
                let response = helpers::build_response_json_error(
                    "Failed to parse component settings, missing Pdforge API Key",
                    500,
                );
                response.send(resp);
                return;
            }
        };

        let body = match extract_request_body(req) {
            Ok(body) => body,
            Err(e) => {
                let response = helpers::build_response_json_error(&e.to_string(), 400);
                response.send(resp);
                return;
            }
        };
        let payload = PdforgeGeneratePdfPayload::new(body, &settings.template_id);

        let pdforge_response = payload.send(&settings.api_key);

        // handle error in case request couldn't be sent
        match pdforge_response {
            Ok(response) => {
                let status_code = response.status_code();
                let response_body =
                    String::from_utf8_lossy(&response.body().unwrap_or_default()).to_string();
                let json_response: serde_json::Value = match serde_json::from_str(&response_body) {
                    Ok(json) => json,
                    Err(_) => {
                        let response = helpers::build_response_json_error(
                            "Failed to parse Pdforge response",
                            500,
                        );
                        response.send(resp);
                        return;
                    }
                };
                let response =
                    helpers::build_response_json(&json_response.to_string(), status_code);
                response.send(resp);
            }
            Err(e) => {
                let response = helpers::build_response_json_error(&e.to_string(), 500);
                response.send(resp)
            }
        }
    }
}

fn extract_request_body(req: IncomingRequest) -> anyhow::Result<serde_json::Value> {
    match req.method() {
        Method::Post => {
            let request_body = match helpers::parse_body(req) {
                Ok(body) => body,
                Err(e) => {
                    return Err(anyhow::anyhow!("Failed to read request body: {e}"));
                }
            };
            let body_json: serde_json::Value = match serde_json::from_slice(&request_body) {
                Ok(json) => json,
                Err(e) => {
                    return Err(anyhow::anyhow!("Failed to parse JSON body: {e}"));
                }
            };
            Ok(body_json)
        }
        _ => Err(anyhow::anyhow!("Unsupported method")),
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Settings {
    pub api_key: String,
    pub template_id: String,
}

impl Settings {
    pub fn from_req(req: &IncomingRequest) -> anyhow::Result<Self> {
        let map = helpers::parse_headers(&IncomingRequest::headers(req));
        Self::new(&map)
    }

    pub fn new(headers: &HashMap<String, Vec<String>>) -> anyhow::Result<Self> {
        let settings = headers
            .get("x-edgee-component-settings")
            .ok_or_else(|| anyhow::anyhow!("Missing 'x-edgee-component-settings' header"))?;

        if settings.len() != 1 {
            return Err(anyhow::anyhow!(
                "Expected exactly one 'x-edgee-component-settings' header, found {}",
                settings.len()
            ));
        }
        let setting = settings[0].clone();
        let setting: HashMap<String, String> = serde_json::from_str(&setting)?;

        let api_key = setting
            .get("api_key")
            .map(String::to_string)
            .ok_or_else(|| anyhow::anyhow!("Missing 'api_key' in settings"))?;

        let template_id = setting
            .get("template_id")
            .map(String::to_string)
            .ok_or_else(|| anyhow::anyhow!("Missing 'template_id' in settings"))?;

        Ok(Self {
            api_key,
            template_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_new() {
        let mut headers = HashMap::new();
        headers.insert(
            "x-edgee-component-settings".to_string(),
            vec![r#"{"api_key": "test_value", "template_id": "test_template_id"}"#.to_string()],
        );

        let settings = Settings::new(&headers).unwrap();
        assert_eq!(settings.api_key, "test_value");
        assert_eq!(settings.template_id, "test_template_id");
    }

    #[test]
    fn test_settings_new_missing_header() {
        let headers = HashMap::new();
        let result = Settings::new(&headers);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Missing 'x-edgee-component-settings' header"
        );
    }

    #[test]
    fn test_settings_new_multiple_headers() {
        let mut headers = HashMap::new();
        headers.insert(
            "x-edgee-component-settings".to_string(),
            vec![
                r#"{"api_key": "test_value"}"#.to_string(),
                r#"{"api_key": "another_value"}"#.to_string(),
            ],
        );
        let result = Settings::new(&headers);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Expected exactly one 'x-edgee-component-settings' header"));
    }

    #[test]
    fn test_settings_new_invalid_json() {
        let mut headers = HashMap::new();
        headers.insert(
            "x-edgee-component-settings".to_string(),
            vec!["not a json".to_string()],
        );
        let result = Settings::new(&headers);
        assert!(result.is_err());
    }
}
