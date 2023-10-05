use crate::business::product_by_id_use_case::product_by_id_use_case;
use chrono::Utc;
use entity::product;
use rocket::tokio;
use sea_orm::prelude::Decimal;
use sea_orm::{DatabaseBackend, MockDatabase};

#[tokio::test]
async fn should_properly_find_product_by_id() -> Result<(), ()> {
    let created_at = Utc::now().naive_utc();

    let mock_db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([vec![product::Model {
            id: "ABCDE".to_string(),
            name: "Bubblegum".to_string(),
            value: Decimal::new(12, 3),
            created_at,
        }]])
        .into_connection();

    assert_eq!(
        Some(product::Model {
            id: "ABCDE".to_string(),
            name: "Bubblegum".to_string(),
            value: Decimal::new(12, 3),
            created_at,
        }),
        product_by_id_use_case("ABCDE", &mock_db).await
    );

    Ok(())
}
