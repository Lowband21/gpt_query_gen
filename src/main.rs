use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::env;
use dotenv::dotenv;
use actix_web::middleware::Logger;

mod routes;

use crate::routes::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Here");
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();


    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(routes::init_routes)
            .service(actix_files::Files::new("/", "./client/public").index_file("index.html"))
            .service(
                web::scope("/api")
                    .route("/chat", web::post().to(chat)),
                    )
            .default_service(
                web::route().to(move |req: HttpRequest| {
                    let path = req.path().to_owned();
                    async move {
                        if path.starts_with("/api") {
                            HttpResponse::NotFound().finish()
                        } else {
                            match actix_files::NamedFile::open("./client/public/index.html") {
                                Ok(file) => file.into_response(&req),
                                Err(e) => {
                                    println!("Error opening file: {:?}", e);
                                    HttpResponse::InternalServerError().finish()
                                },
                            }
                        }
                    }
                })
            )
    })
    .bind(format!("0.0.0.0:{}", env::var("PORT").unwrap_or_else(|_| "5000".to_string())))?
    .run()
    .await
}
