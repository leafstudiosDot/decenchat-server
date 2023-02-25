use actix_web::{get, middleware, web, App, HttpServer, Result, Responder};
use actix_files::NamedFile;
use std::path::PathBuf;
use serde::Serialize;

mod members;

#[derive(Serialize)]
struct BrowserNotice {
    notice: String,
}
#[get("/")]
async fn index() -> impl Responder {
    let res = BrowserNotice {
        notice: "Please open this address with Decensha Client".to_string(),
    };
    return web::Json(res)
}

#[get("/icon.png")]
async fn icon() -> Result<NamedFile> {
    let path: PathBuf = "./assets/icon.png".parse().unwrap();
    Ok(NamedFile::open(path)?)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("Your Decensha Server port for development is 7810");

    HttpServer::new(|| {
        let app = App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(index)
            .service(icon)
            .service(web::resource("/members/join_server").route(web::post().to(members::server_join)));
        app
    })
    .bind(("0.0.0.0", 7810))?
    .run()
    .await
}