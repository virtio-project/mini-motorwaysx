use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use anyhow::Result;
use internal::api::*;
use internal::config::CONFIG;

#[actix_web::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(report)
                    .service(stats)
                    .service(screenshot),
            )
            .service(actix_files::Files::new("/", "frontend/build/").index_file("index.html"))
    })
    .bind(&CONFIG.actix.bind)?
    .run()
    .await?;

    Ok(())
}
