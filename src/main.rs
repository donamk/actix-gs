use awc::Client;
use log::LevelFilter;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct MyIp {
    origin: String,
}

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::new()
        .filter(None, LevelFilter::Debug)
        .init();

    let client = Client::default();

    let mut res = client
        .get("http://httpbin.org/ip")
        .insert_header(("Content-Type", "application/json"))
        .send()
        .await?;
    log::debug!("Response: {:?}", res);

    let raw_body = res.body().await?;
    log::debug!("Raw Body: {:?}", raw_body);

    // let json_raw = r#"{ "origin": "123.123.123.133" }"#;
    // let j: MyIp = serde_json::from_str(&json_raw).unwrap();
    // log::debug!("Json Body: {:?}", j.origin);

    // let json_body = res.json::<MyIp>().await?;
    let json_body: MyIp = serde_json::from_slice(&raw_body)?;
    log::debug!("Json Body: {:?}", json_body.origin);

    return Ok::<(), Box<dyn Error>>(());
}
