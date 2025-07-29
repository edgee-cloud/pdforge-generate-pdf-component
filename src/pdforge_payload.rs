use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PdforgeGeneratePdfPayload {
    #[serde(rename = "templateId")]
    pub template_id: String,
    #[serde(rename = "data")]
    pub pdf_data: serde_json::Value,
}

impl PdforgeGeneratePdfPayload {
    pub fn new(pdf_data: &serde_json::Value, template_id: &str) -> Self {
        Self {
            pdf_data: pdf_data.clone(),
            template_id: template_id.to_string(),
        }
    }

    #[cfg(not(test))]
    const PDFORGE_ENDPOINT: &str = "https://api.pdforge.com/v1/pdf/sync";

    #[cfg(not(test))]
    pub fn send(&self, api_key: &str) -> anyhow::Result<waki::Response> {
        // the body is url-encoded

        // call the PDForge API
        let client = waki::Client::new();
        let response = client
            .post(Self::PDFORGE_ENDPOINT)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {api_key}"))
            .body(serde_json::to_string(&self)?)
            .send()?;

        Ok(response)
    }
}
