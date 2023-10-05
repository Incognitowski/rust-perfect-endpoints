mod business;
mod database;
mod gateway;
mod infrastructure;
mod transport;

#[macro_use]
extern crate rocket;

use crate::database::get_connection_pool;
use crate::infrastructure::app_state::AppState;
use dotenvy::dotenv;
use migration::MigratorTrait;
use std::env;
use transport::index_route::index_route as IndexRoute;
use transport::product_by_id_route::product_by_id_route as ProductByIdRoute;
use transport::product_price_quote_route::product_price_quote_route as ProductPriceQuoteRoute;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let database_connection = get_connection_pool().await;

    migration::Migrator::up(&database_connection, None)
        .await
        .expect("Failed to migrate database.");

    let app_state = AppState::new(
        database_connection,
        env::var("PRICE_CONVERSION_HOST")
            .expect("Failed to load PRICE_CONVERSION_HOST environment variable"),
    );

    rocket::build().manage(app_state).mount(
        "/",
        routes![IndexRoute, ProductByIdRoute, ProductPriceQuoteRoute],
    )
}
