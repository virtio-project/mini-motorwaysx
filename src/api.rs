use crate::models::*;
use actix_web::{post, web, HttpResponse};
use actix_multipart::{Multipart, Field, MultipartError};

#[post("/report")]
pub async fn report(report: web::Json<Report>) -> HttpResponse {
    info!("{:?}", report);
    HttpResponse::NoContent().finish()
}

#[post("/stats")]
pub async fn stats(statics: web::Json<Statics>) -> HttpResponse {
    info!("{:?}", statics);
    HttpResponse::NoContent().finish()
}

#[post("/screenshot")]
pub async fn screenshot(
    steam_id: web::Header<SteamId>,
    steam_name: web::Header<SteamName>,
    screenshot_name: web::Header<ScreenshotName>,
    data: web::Bytes,
) -> HttpResponse {
    info!("{}#{}: {} ({} bytes)", steam_name, steam_id, screenshot_name, data.len());
    HttpResponse::NoContent().finish()
}
