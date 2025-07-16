use serde::{Deserialize, Serialize};

const PDFORGE_ENDPOINT: &str = "https://api.pdforge.com/v1/pdf/sync";

#[derive(Serialize, Deserialize)]
pub struct PdforgeGeneratePdfPayload {
    #[serde(rename = "templateId")]
    pub template_id: String,
    #[serde(rename = "data")]
    pub pdf_data: serde_json::Value,
}

impl PdforgeGeneratePdfPayload {
    pub fn new(pdf_data: serde_json::Value, template_id: &str) -> Self {
        Self {
            pdf_data,
            template_id: template_id.to_string(),
        }
    }

    pub fn send(&self, api_key: &str) -> anyhow::Result<waki::Response> {
        // the body is url-encoded

        // call the Stripe API
        let client = waki::Client::new();
        let response = client
            .post(PDFORGE_ENDPOINT)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {api_key}"))
            .body(serde_json::to_string(&self)?)
            .send()?;

        Ok(response)
    }
}
