#[derive(serde::Serialize, serde::Deserialize)]
pub struct CheckHealthResponse {
    pub status: u16,
    pub response: String,
}
