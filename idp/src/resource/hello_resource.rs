//! Hello and Resource.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestReqBody {
    name: String,
    number: i32,
}

pub async fn hello_handler(body: web::Json<TestReqBody>) -> actix_web::Result<HttpResponse> {
    println!("model: {:?}", &body);
    Ok(HttpResponse::Ok().json(body.0))
}
