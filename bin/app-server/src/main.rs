use actix_web::{get, web, App, HttpServer, Responder};
use person::{Person};

#[get("/{name}/{height_cm}/index.html")]
async fn index(path: web::Path<Person>) -> impl Responder {
    format!("{}", *path)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listen_addr = std::env::var("LISTEN_ADDR").unwrap();
    println!("Listening on {}", listen_addr);
    HttpServer::new(|| App::new().service(index))
        .bind(listen_addr)?
        .run()
        .await
}