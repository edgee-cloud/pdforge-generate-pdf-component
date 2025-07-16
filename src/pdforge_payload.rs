use serde_json::json;

const PDFORGE_ENDPOINT: &str = "https://api.pdforge.com/v1/pdf/sync";

pub struct PdforgeGeneratePdfPayload {
    pub pdf_data: serde_json::Value,
    pub template_id: String,
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
        let template_id = self.template_id.clone();
        let pdf_data = self.pdf_data.clone();
        let body = json!({
            "templateId": template_id,
            "data": pdf_data,
        })
        .to_string();

        // call the Stripe API
        let client = waki::Client::new();
        let response = client
            .post(PDFORGE_ENDPOINT)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {api_key}"))
            .body(body)
            .send()?;

        Ok(response)
    }
}
