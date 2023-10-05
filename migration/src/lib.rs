pub use sea_orm_migration::prelude::*;

mod m20230926_000001_create_table_product;
mod m20230926_000002_create_table_shopping_cart;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230926_000001_create_table_product::Migration),
            Box::new(m20230926_000002_create_table_shopping_cart::Migration),
        ]
    }
}
