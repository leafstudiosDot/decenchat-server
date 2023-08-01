use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use serde_json::json;
use serde_json::{Number, Value};

pub async fn details() -> HttpResponse {

    let servdetfile = "./assets/details.json".to_string();

    let mut server_details = {
        let text = std::fs::read_to_string(&servdetfile).unwrap();
        serde_json::from_str::<Value>(&text).unwrap()
    };

    let res = json!({
        "title": server_details["title"].as_str(),
    });

    HttpResponse::Ok().json(res)
}