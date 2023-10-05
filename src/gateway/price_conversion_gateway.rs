use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

#[async_trait::async_trait]
pub trait PriceConversionGateway: Sync {
    async fn convert_to_btc(&self, usd_value: &Decimal) -> Decimal;
}

pub struct PriceConversionClient<'r> {
    service_base_url: &'r str,
}

impl<'r> PriceConversionClient<'r> {
    pub fn new(base_url: &str) -> PriceConversionClient {
        PriceConversionClient {
            service_base_url: base_url,
        }
    }
}

#[async_trait::async_trait]
impl<'r> PriceConversionGateway for PriceConversionClient<'r> {
    async fn convert_to_btc(&self, usd_value: &Decimal) -> Decimal {
        let url = format!("{}/usd-to-btc", self.service_base_url);
        let client = reqwest::Client::new();
        let request_body = PriceConversionRequest { usd: usd_value };
        let response = client.post(url).json(&request_body).send().await.unwrap();
        let price_conversion_response = response.json::<PriceConversionResponse>().await.unwrap();
        price_conversion_response.btc
    }
}

#[derive(Debug, Serialize)]
struct PriceConversionRequest<'r> {
    pub usd: &'r Decimal,
}

#[derive(Debug, Deserialize)]
struct PriceConversionResponse {
    pub btc: Decimal,
}
