use crate::infrastructure::default_response::DefaultResponse;
use rocket::http::Status;
use rocket::outcome::Outcome::{Failure, Success};
use rocket::request::{FromRequest, Outcome, Request};
use std::env;

pub struct ApiKey {
    pub key: String,
}

impl ApiKey {
    pub fn new(key: &str) -> ApiKey {
        ApiKey {
            key: key.to_string(),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = DefaultResponse;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let expected_key = env::var("API_KEY").expect("API_KEY must be set");
        let auth_header = request.headers().get_one("Authorization").unwrap_or("");
        if auth_header == expected_key {
            Success(ApiKey::new(auth_header))
        } else {
            Failure((
                Status::Unauthorized,
                DefaultResponse::basic("Authentication failed"),
            ))
        }
    }
}
