use actix_web::web::{Data, Json};
use actix_web::HttpResponse;
use sqlx::PgPool;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CreateCategory {
    name: String,
    #[serde(default)]
    parent_id: u32,
}

pub async fn create_category(pool: Data<PgPool>, command: Json<CreateCategory>) -> HttpResponse {
    println!("{:#?}", command);

    HttpResponse::Ok().finish()
}
