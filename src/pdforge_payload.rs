const PDFORGE_ENDPOINT: &str = "https://api.pdforge.com/v1/pdf/sync";

pub struct PdforgeGeneratePdfPayload {
    pub body: serde_json::Value,
}

impl PdforgeGeneratePdfPayload {
    pub fn new(body: serde_json::Value) -> Self {
        Self { body }
    }

    pub fn send(&self, api_key: &str) -> anyhow::Result<waki::Response> {
        // the body is url-encoded
        let body = self.body.to_string();

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
