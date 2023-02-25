use actix_web::{post, web, HttpResponse, Result};
use serde::Serialize;

#[derive(Serialize)]
struct ResultStruct {
    notice: String,
}

pub async fn server_join() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ResultStruct {
        notice: "".to_string(),
    }))
}