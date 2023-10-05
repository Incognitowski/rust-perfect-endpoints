use crate::business::product_price_quote_use_case::{
    product_price_quote_use_case, ProductQuoteResponse,
};
use crate::gateway::price_conversion_gateway::PriceConversionClient;
use crate::infrastructure::api_auth::ApiKey;
use crate::infrastructure::app_state::AppState;
use rocket::State;

#[get("/products/<product_id>/quote")]
pub async fn product_price_quote_route(
    _api_key: ApiKey,
    app_state: &State<AppState>,
    product_id: &str,
) -> Option<ProductQuoteResponse> {
    let database_connection = &app_state.inner().db_connection;
    let price_conversion_client =
        PriceConversionClient::new(&app_state.inner().price_conversion_host);
    product_price_quote_use_case(product_id, database_connection, &price_conversion_client).await
}
