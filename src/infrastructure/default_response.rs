use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DefaultResponse {
    pub message: String,
    pub details: Vec<String>,
}

impl DefaultResponse {
    pub fn basic(message: &str) -> DefaultResponse {
        DefaultResponse {
            message: message.to_string(),
            details: vec![],
        }
    }
}
