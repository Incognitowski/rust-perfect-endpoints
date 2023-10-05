use rocket::{Request, Response, response};
use rocket::http::ContentType;
use rocket::response::Responder;
use crate::gateway::price_conversion_gateway::PriceConversionGateway;
use entity::prelude::Product;
use entity::product::Model as ProductModel;
use rocket::serde::Serialize;
use sea_orm::prelude::Decimal;
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ProductQuoteResponse {
    pub product: ProductModel,
    pub btc_quote: Decimal,
}

impl<'r> Responder<'r, 'static> for ProductQuoteResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let body = serde_json::to_string(&self).unwrap();
        Response::build_from(body.respond_to(req)?)
            .header(ContentType::new("application", "json"))
            .ok()
    }
}

pub async fn product_price_quote_use_case(
    product_id: &str,
    database_connection: &DatabaseConnection,
    price_converter: &dyn PriceConversionGateway,
) -> Option<ProductQuoteResponse> {
    let product = Product::find_by_id(product_id)
        .one(database_connection)
        .await
        .unwrap();

    let product = match product {
        None => return None,
        Some(p) => p,
    };

    let btc_quote = price_converter.convert_to_btc(&product.value).await;

    Some(ProductQuoteResponse { product, btc_quote })
}
