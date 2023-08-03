use actix_web::{web, HttpResponse, Result, get, HttpRequest, Error};
use actix_files::NamedFile;
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::io::{self, Read};

use serde_json::json;
use serde_json::{Number, Value};

use regex::Regex;

use crate::SECURED_CERT;

fn limit_characters(input_str: Option<&str>, limit: usize) -> &str {
    match input_str {
        Some(str) => &str[..std::cmp::min(str.len(), limit)],
        None => "",
    }
}

fn openssl_extract_subject_fields(buffer: &str) -> Option<(String, String, String, String, String, String)> {
    let re = Regex::new(r"subject\s*=\s*C\s*=\s*(.*?),\s*ST\s*=\s*(.*?),\s*L\s*=\s*(.*?),\s*O\s*=\s*(.*?),\s*CN\s*=\s*(.*?),\s*emailAddress\s*=\s*(.*?)\s*").unwrap();

    if let Some(captures) = re.captures(buffer) {
        let country = captures.get(1).unwrap().as_str().to_string();
        let state = captures.get(2).unwrap().as_str().to_string();
        let locality = captures.get(3).unwrap().as_str().to_string();
        let common_name = captures.get(5).unwrap().as_str().to_string();
        let organization = captures.get(4).unwrap().as_str().to_string();
        let email_address = captures.get(6).unwrap().as_str().to_string();

        Some((country, state, locality, common_name, organization, email_address))
    } else {
        None
    }
}

fn certified(server_details: serde_json::Value) -> Result<(serde_json::Value), actix_web::Error> {
    if unsafe {SECURED_CERT} == true {
        let openssl_o = Command::new("openssl")
            .arg("x509")
            .arg("-noout")
            .arg("-subject")
            .arg("-in")
            .arg("./assets/certs/cert.pem".to_string())
            .stdout(Stdio::piped())
            .spawn()
            .expect("OpenSSL Error occurred");

        let mut buffer = String::new();
        match openssl_o.stdout.expect("Failed to open stdout").read_to_string(&mut buffer) {
            Ok(_) => {
                println!("Output from OpenSSL: {}", buffer);
    
                if buffer.is_empty() {
                    println!("Empty Buffer from OpenSSL");
                    return Ok(json!({ "certified": false }));
                } else {
                    if let Some((country, state, locality, common_name, organization, email_address)) = openssl_extract_subject_fields(&buffer) {
                        println!("Country: {}", country);
                        println!("State: {}", state);
                        println!("Locality: {}", locality);
                        println!("Common Name: {}", common_name);
                        println!("Organization: {}", organization);
                        println!("Email Address: {}", email_address);

                        if (Some(common_name.to_string()) != server_details["serverid"].as_str().map(|s| s.to_string())) {
                            println!("Common Name does not match serverid");
                            return Ok(json!({ "certified": false }));
                        } else {
                            return Ok(json!({ 
                                "certified": true, 
                                "country": country, 
                                "state": state, 
                                "locality": locality,
                                "organization": organization,
                                "common_name": common_name,
                                "email_address": email_address,
                            }));
                        }
                    } else {
                        println!("Error extracting subject fields from OpenSSL output");
                        println!("OpenSSL Output: {}", buffer);
                        return Ok(json!({ "certified": false }));
                    }
                }
            }
            Err(error) => {
                println!("Error reading OpenSSL output: {}", error);
                return Ok(json!({ "certified": false }));
            }
        }
    } else {
        Ok(json!({
            "certified": false,
        }))
    }
}

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
        "serverid": limit_characters(server_details["serverid"].as_str(), 32),
        "signinmethod": server_details["signinmethod"].as_str(),
        "nsfwfocused": server_details["nsfwfocused"].as_bool(),
        "connectionmethod": server_details["connectaccountmethod"].as_array().unwrap(),
        "serverversion": server_details["serverversion"].to_string().parse::<i32>().unwrap(),
        "rules": server_details["rules"].as_array().unwrap(),
        "certified": certified(server_details).unwrap(),
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
