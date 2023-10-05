use crate::business::product_price_quote_use_case::{
    product_price_quote_use_case, ProductQuoteResponse,
};
use crate::gateway::price_conversion_gateway::PriceConversionGateway;
use chrono::Utc;
use entity::product;
use rocket::tokio;
use sea_orm::prelude::Decimal;
use sea_orm::{DatabaseBackend, MockDatabase};

#[tokio::test]
async fn should_properly_quote_product() -> Result<(), ()> {
    let created_at = Utc::now().naive_utc();

    let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([vec![product::Model {
            id: "a".to_string(),
            name: "Tennis Ball".to_string(),
            value: Decimal::new(12, 3),
            created_at,
        }]])
        .into_connection();

    assert_eq!(
        Some(ProductQuoteResponse {
            product: product::Model {
                id: "a".to_string(),
                name: "Tennis Ball".to_string(),
                value: Decimal::new(12, 3),
                created_at,
            },
            btc_quote: Decimal::new(123, 12),
        }),
        product_price_quote_use_case("ABCDE", &mock_db, &PriceConversionGatewayMock).await
    );

    Ok(())
}

#[tokio::test]
async fn should_return_none_when_not_able_to_find_product() -> Result<(), ()> {
    let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([vec![] as Vec<product::Model>])
        .into_connection();

    assert_eq!(
        None,
        product_price_quote_use_case("ABCDE", &mock_db, &PriceConversionGatewayMock).await
    );

    Ok(())
}

struct PriceConversionGatewayMock;

#[async_trait]
impl PriceConversionGateway for PriceConversionGatewayMock {
    async fn convert_to_btc(&self, _usd_value: &Decimal) -> Decimal {
        Decimal::new(123, 12)
    }
}
