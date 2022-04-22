use actix_web::{get, web, Responder};

#[get("/{id}/{name}/index.html")]
async fn hello_html_handler(params: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = params.into_inner();
    format!("Hello {}! id:{}", name, id)
}
