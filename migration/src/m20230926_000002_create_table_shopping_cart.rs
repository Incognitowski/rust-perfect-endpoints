use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(r#"
            CREATE TABLE IF NOT EXISTS shopping_cart (
              "id" varchar(30) PRIMARY KEY,
              "value" DECIMAL(15,2) NOT NULL,
              "tax_id" varchar(11) NOT NULL,
              "status" varchar(50) NOT NULL,
              "created_at" TIMESTAMP NOT NULL,
              "updated_at" TIMESTAMP
            );

            CREATE INDEX IF NOT EXISTS idx_shopping_cart_tax_id ON shopping_cart(tax_id);
            CREATE INDEX IF NOT EXISTS idx_shopping_cart_status ON shopping_cart(status);

            CREATE TABLE IF NOT EXISTS shopping_cart_product (
              "id" varchar(30) PRIMARY KEY,
              "shopping_cart_id" varchar(30) NOT NULL,
              "product_id" varchar(30) NOT NULL,
              "value" DECIMAL(15,2) NOT NULL,
              "created_at" TIMESTAMP NOT NULL,
              CONSTRAINT fk_shopping_cart_product_shopping_cart FOREIGN KEY(shopping_cart_id) REFERENCES shopping_cart(id),
              CONSTRAINT fk_shopping_cart_product_product FOREIGN KEY(product_id) REFERENCES product(id)
            );
        "#).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"
            DROP TABLE shopping_cart_product;
            DROP INDEX idx_shopping_cart_tax_id;
            DROP INDEX idx_shopping_cart_status;
            DROP TABLE shopping_cart;
        "#,
        )
        .await?;
        Ok(())
    }
}
