use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
pub struct JoinRequestStruct {
    key: String,
    name: Option<String>,

}
pub async fn server_join(newmem: web::Json<JoinRequestStruct>) -> Result<HttpResponse> {

    let res = json!({
        "notice": newmem.key.to_string(),
    });

    Ok(HttpResponse::Ok().content_type("application/json").json(res))
}

pub async fn server_left() -> Result<HttpResponse> {

    let res = json!({
        "notice": "Left the server".to_string(),
    });

    Ok(HttpResponse::Ok().content_type("application/json").json(res))
}