use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub valid: bool,
    pub data: Vec<User>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub username: String,
    pub acc: UserAcc,
}

#[derive(Debug, Deserialize)]
pub struct UserAcc {
    pub admin: bool,
    pub enabled: bool,
}

// pub fn dummy_user_info() -> ApiResponse {
//     return ApiResponse {
//         valid: false,
//         data: vec![],
//     };
// }
