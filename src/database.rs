use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;

pub async fn get_connection_pool() -> DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut opt = ConnectOptions::new(database_url.to_owned());
    opt.max_connections(100)
        .min_connections(20)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .set_schema_search_path("public");

    Database::connect(opt).await.unwrap()
}
