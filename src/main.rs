use actix_web::{get, middleware, web, http, App, HttpServer, Result, HttpResponse};
use actix_files::NamedFile;
use std::path::PathBuf;
use serde_json::json;
use dotenv::dotenv;
use actix_cors::Cors;

mod members;
mod server;

mod db;
use postgres::{Client, Error, NoTls};
pub use crate::db::db_postgres;

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