use std::io::Cursor;

use gloo_net::http::Request;
use serde::Deserialize;
use tsify::Tsify;
use wasm_bindgen::JsError;

use crate::errors::TimelineError;

#[derive(Deserialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct InfluxdbConfig {
    influxdb_url: String,
    influxdb_org: String,
    influxdb_token: String,
    flux_query: String,
}

pub(crate) async fn get_data(config: &InfluxdbConfig) -> Result<Cursor<Vec<u8>>, JsError> {
    let token = format!("Token {}", config.influxdb_token);
    let request = Request::post(&config.influxdb_url)
        .query([("org", &config.influxdb_org)])
        .header("Accept", "application/csv")
        .header("Authorization", &token)
        .header("Content-Type", "application/vnd.flux")
        .body(&config.flux_query);
    let response = request.send().await?;
    if !response.ok() {
        return Err(TimelineError::ResponseStatus(response.status()).into());
    }
    let data = response.binary().await?;
    Ok(Cursor::new(data))
}
