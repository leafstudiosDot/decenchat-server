use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use serde_json::json;
use serde_json::{Number, Value};

pub async fn details() -> HttpResponse {

    let servdetfile = "./assets/details.json".to_string();

    let server_details = {
        let text = std::fs::read_to_string(&servdetfile).unwrap();
        serde_json::from_str::<Value>(&text).unwrap()
    };

    let res = json!({
        "title": server_details["title"].as_str(),
        "serverid": server_details["serverid"].as_str(),
        "signinmethod": server_details["serverid"].as_str(),
        "nsfwfocused": server_details["nsfwfocused"].as_bool(),
        "connectionmethod": server_details["connectaccountmethod"].as_array().unwrap(),
        "serverversion": server_details["serverversion"].to_string().parse::<i32>().unwrap(),
    });

    HttpResponse::Ok().json(res)
}