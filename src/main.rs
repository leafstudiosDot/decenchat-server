use actix_web::{get, middleware, web, App, HttpServer, Result, HttpResponse};
use actix_files::NamedFile;
use std::path::PathBuf;
use serde_json::json;
use dotenv::dotenv;

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

#[get("/icon.png")]
async fn icon() -> Result<NamedFile> {
    let path: PathBuf = "./assets/icon.png".parse().unwrap();
    Ok(NamedFile::open(path)?)
}


fn main() -> std::io::Result<()> {
    println!("Starting Decensha Server...");
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    //println!("{}", db::db_chose());

    match db::db_chose().as_str() {
        "POSTGRES" => {
            println!("Your database will use Postgres");
            //db_postgres::pg_connect();
            let db_client = Client::connect(
                format!("postgresql://{}:{}@{}:{}/postgres", db::db_postgres::pg_user(), db::db_postgres::pg_pass(), db::db_postgres::pg_host(), db::db_postgres::pg_port()).as_str(),
                NoTls,
            );
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
            .service(index)
            .service(icon)
            .service(web::scope("/members")
                .service(members::server_join)
                .service(members::server_left)
            )
            .service(web::resource("/server/details").route(web::get().to(server::details)));
        app
    })
    .bind(("0.0.0.0", 7810))?
    .run()
    .await
}