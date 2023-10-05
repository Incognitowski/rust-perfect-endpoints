use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"
            CREATE TABLE IF NOT EXISTS product (
              "id" varchar(30) PRIMARY KEY,
              "name" text NOT NULL,
              "value" DECIMAL(15,2) NOT NULL,
              "created_at" TIMESTAMP NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_product_name ON product(name);
        "#,
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"
            DROP INDEX idx_product_name;
            DROP TABLE product;
        "#,
        )
        .await?;
        Ok(())
    }
}
