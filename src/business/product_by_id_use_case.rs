use entity::prelude::Product;
use entity::product::Model as ProductModel;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn product_by_id_use_case(
    product_id: &str,
    database_connection: &DatabaseConnection,
) -> Option<ProductModel> {
    Product::find_by_id(product_id)
        .one(database_connection)
        .await
        .unwrap()
}
