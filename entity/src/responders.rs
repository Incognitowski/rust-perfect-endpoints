use rocket::{Request, response, Response};
use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::serde::json::serde_json;
use crate::product::Model as ProductModel;

impl<'r> Responder<'r, 'static> for ProductModel {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let body = serde_json::to_string(&self).unwrap();
        Response::build_from(body.respond_to(req)?)
            .header(ContentType::new("application", "json"))
            .ok()
    }
}