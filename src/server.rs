use actix_web::{web, HttpResponse, Result, get, HttpRequest, Error};
use actix_files::NamedFile;
use serde::{Deserialize, Serialize};

use serde_json::json;
use serde_json::{Number, Value};

use crate::SECURED_CERT;

#[get("/details")]
pub async fn details() -> Result<HttpResponse> {

    let servdetfile = "./assets/details.json".to_string();

    let server_details = {
        let text = std::fs::read_to_string(&servdetfile).unwrap();
        serde_json::from_str::<Value>(&text).unwrap()
    };

    let res = json!({
        "title": server_details["title"].as_str(),
        "description": server_details["description"].as_str(),
        "serverid": server_details["serverid"].as_str(),
        "signinmethod": server_details["serverid"].as_str(),
        "nsfwfocused": server_details["nsfwfocused"].as_bool(),
        "connectionmethod": server_details["connectaccountmethod"].as_array().unwrap(),
        "serverversion": server_details["serverversion"].to_string().parse::<i32>().unwrap(),
        "rules": server_details["rules"].as_array().unwrap(),
        "certified": unsafe { SECURED_CERT },
    });

    Ok(HttpResponse::Ok().content_type("application/json").json(res))
}

#[get("/icon")]
pub async fn icon(req: HttpRequest) -> Result<HttpResponse, Error> {
    match std::fs::read("./assets/icon.png") {
        Ok(icon) => {
            Ok(HttpResponse::Ok().content_type("image/png").body(icon))
        },
        Err(e) => {
            println!("icon({}): error, {:?}", req.match_info().query("id").parse::<String>().unwrap(), e);
            Err(actix_web::Error::from(e))
        }
    }
}
