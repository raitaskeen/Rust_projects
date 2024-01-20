// use actix_web::{web, App, HttpServer, HttpResponse};

// async fn index() -> HttpResponse {
//     HttpResponse::Ok().body("Hello, young developer! This is your first Rust backend.")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new().service(web::resource("/").to(index))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }
