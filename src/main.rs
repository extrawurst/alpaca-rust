use rest_api::{api, endpoint};
use std::env;

//TODO: #[api_root("base_url")]
#[api]
trait AlpacaApi {
    /// see https://docs.alpaca.markets/api-documentation/api-v2/account/
    #[endpoint("account")]
    fn get_account(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>>;

    /// see https://docs.alpaca.markets/api-documentation/api-v2/account-activities/
    #[endpoint("account/activities/FILL")]
    fn account_activities(
        &self, //activity_type: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>>;

    /// see https://docs.alpaca.markets/api-documentation/api-v2/clock/
    fn clock(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
}

fn main() {
    let mut headers = http::HeaderMap::new();

    headers.insert(
        "APCA-API-KEY-ID",
        env::var("APCA_API_KEY_ID").unwrap().parse().unwrap(),
    );
    headers.insert(
        "APCA-API-SECRET-KEY",
        env::var("APCA_API_SECRET_KEY").unwrap().parse().unwrap(),
    );

    let api: &dyn AlpacaApi = &AlpacaApiRestClient {
        headers: headers,
        base_url: "https://paper-api.alpaca.markets/v2/".parse().unwrap(),
    };

    let acc = api.get_account().unwrap();

    println!("{:#?}", acc);

    let act = api.account_activities().unwrap();

    println!("{:#?}", act);

    let clc = api.clock().unwrap();

    println!("{:#?}", clc);
}
