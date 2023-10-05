use crate::business::product_by_id_use_case::product_by_id_use_case;
use crate::infrastructure::api_auth::ApiKey;
use crate::infrastructure::app_state::AppState;
use entity::product::Model as ProductModel;
use rocket::State;

#[get("/products/<product_id>")]
pub async fn product_by_id_route(
    _api_key: ApiKey,
    database_connection: &State<AppState>,
    product_id: &str,
) -> Option<ProductModel> {
    product_by_id_use_case(product_id, &database_connection.inner().db_connection).await
}
