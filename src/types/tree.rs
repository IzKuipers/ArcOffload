use serde::Deserialize;

use super::api::ApiResponse;

#[derive(Debug, Deserialize)]
pub struct TreeEntry {
    #[serde(rename = "scopedPath")]
    pub scoped_path: String,
    pub mime: String,
    pub filename: String,
}

pub fn tree_response_dummy() -> ApiResponse<TreeEntry> {
    return ApiResponse {
        valid: false,
        data: vec![],
    };
}
