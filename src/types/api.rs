use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub valid: bool,
    pub data: Vec<T>,
}
