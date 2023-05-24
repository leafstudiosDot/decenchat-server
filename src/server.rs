use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub async fn details() -> HttpResponse {

    let res = json!({
        "notice": "NO MORE".to_string(),
    });

    HttpResponse::Ok().json(res)
}