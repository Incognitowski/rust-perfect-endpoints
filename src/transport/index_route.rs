use crate::infrastructure::api_auth::ApiKey;
use crate::infrastructure::default_response::DefaultResponse;
use rocket::serde::json::Json;

#[get("/")]
pub async fn index_route(_api_key: ApiKey) -> Json<DefaultResponse> {
    Json(DefaultResponse::basic("Server Up"))
}
