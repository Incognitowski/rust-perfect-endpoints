use sea_orm::DatabaseConnection;

pub struct AppState {
    pub db_connection: DatabaseConnection,
    pub price_conversion_host: String,
}

impl AppState {
    pub fn new(db_connection: DatabaseConnection, price_conversion_host: String) -> AppState {
        AppState {
            db_connection,
            price_conversion_host,
        }
    }
}
