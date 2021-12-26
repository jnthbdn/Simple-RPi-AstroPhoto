#[allow(dead_code)]
#[allow(unused_imports)]

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(fs::Files::new("/", "static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}