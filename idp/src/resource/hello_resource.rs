use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestObj {
    name: String,
    number: i32,
}

pub async fn hello_handler(item: web::Json<TestObj>) -> actix_web::Result<HttpResponse> {
    println!("model: {:?}", &item);
    Ok(HttpResponse::Ok().json(item.0))
}
