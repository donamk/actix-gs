use awc::Client;
use log::LevelFilter;

#[actix_rt::main]
async fn main() {
    env_logger::Builder::new()
        .filter(None, LevelFilter::Debug)
        .init();

    let client = Client::default();

    let res = client
        .get("http://httpbin.org/ip")
        .insert_header(("Content-Type", "application/json"))
        .send()
        .await;

    if res.is_err() {
        log::error!("Wikipedia did not return expected image");
    }

    let body = res.unwrap().body().await.unwrap();

    log::debug!("Response: {:?}", body);
}
