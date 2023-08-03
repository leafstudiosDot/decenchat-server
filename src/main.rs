use actix_web::{get, middleware, web, http, App, HttpServer, Result, HttpResponse};
use serde_json::json;
use dotenv::dotenv;
use actix_cors::Cors;

use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use rustls::Certificate;
use x509_parser::pem::Pem;
use x509_parser::x509::X509Version;
use x509_parser::pem::pem_to_der;
use x509_parser::parse_x509_der;

mod members;
mod server;

mod db;
use postgres::{Client, Error, NoTls};
pub use crate::db::db_postgres;

pub static mut SECURED_CERT: bool = false;
static IGCA_PEM: &str = "./assets/certs/cert.pem";

#[get("/")]
async fn index() -> HttpResponse {
    let res = json!({
        "notice": "Please open this address with Decensha Client".to_string(),
    });
    HttpResponse::Ok().json(res)
}


fn main() -> std::io::Result<()> {
    println!("Starting Decensha Server...");
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    if Path::new(IGCA_PEM).exists() == true {
        println!("Entering Secured Conn Mode...");
        let data = std::fs::read(IGCA_PEM).expect("Could not read file");
        /*for pem in Pem::iter_from_buffer(&data) {
            let pem = pem.expect("Reading next PEM block failed");
            let x509 = pem.parse_x509().expect("X.509: decoding DER failed");
            assert_eq!(x509.tbs_certificate.version, X509Version::V3);
        }*/

        let res = pem_to_der(&data);
        match res {
            Ok((rem, pem)) => {
                assert!(rem.is_empty());
                //
                assert_eq!(pem.label, String::from("CERTIFICATE"));
                //
                let res_x509 = parse_x509_der(&pem.contents);
                assert!(res_x509.is_ok());
                unsafe { SECURED_CERT = true };
            },
            _ => panic!("PEM parsing failed: {:?}", res),
        }
    }


    match db::db_chose().as_str() {
        "POSTGRES" => {
            println!("Your database will use Postgres");
            //db_postgres::pg_connect();
            run_server()
        },
        _ => {
            println!("No database chosen");
            std::process::exit(0);
        }
    }

}

#[actix_web::main]
async fn run_server() -> std::io::Result<()> {
    println!("Your Decensha Server port for development is 7810");
    HttpServer::new(|| {

        let app = App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(Cors::default().allow_any_origin().send_wildcard())
            .service(index)
            .service(web::scope("/members")
                .service(members::server_join)
                .service(members::server_left)
            )
            .service(web::scope("/server")
                .service(server::details)
                .service(server::icon)
            );
        app
    })
    .bind(("0.0.0.0", 7810))?
    .run()
    .await
}