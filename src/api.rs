use crate::models::*;
use actix_web::{post, web, HttpResponse};
use tokio::fs;
use std::path::PathBuf;
use std::str::FromStr;
use crate::config::CONFIG;
use std::time::{SystemTime, UNIX_EPOCH};

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
    info!(
        "{}#{}: {} ({} bytes)",
        steam_name,
        steam_id,
        screenshot_name,
        data.len()
    );
    let base_path = PathBuf::from_str(CONFIG.storage.screenshots.as_str())
        .unwrap()
        .join(steam_id.to_string());
    fs::create_dir_all(&base_path).await.unwrap();
    if let Err(e) = fs::write(&base_path.join(format!("{}.jpg", get_epoch_ms())), data).await {
        // swallow error and continue
        error!("{}", e);
    }
    HttpResponse::NoContent().finish()
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}