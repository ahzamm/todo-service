use actix_web::{web, Responder};
// mod models;
use crate::models::Status;

pub async fn status() -> impl Responder {
  web::HttpResponse::Ok().json(Status {
      status: "Up".to_string(),
  })
}