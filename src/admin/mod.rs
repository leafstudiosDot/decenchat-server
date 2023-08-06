use actix_web::{get, Result, HttpResponse};
use serde_json::json;

#[get("/init")]
pub async fn index_admin() -> Result<HttpResponse> {
    let res = json!({
        "notice": "good-admin".to_string(),
    });
    Ok(HttpResponse::Ok().content_type("application/json").json(res))
}