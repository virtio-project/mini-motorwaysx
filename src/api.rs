use crate::models::*;
use actix_web::{post, web, HttpResponse};

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
