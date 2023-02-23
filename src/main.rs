use actix_web::{get, post, middleware, web, App, HttpRequest, HttpServer, Result, Responder};
use serde::Serialize;

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


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("Your Decensha Server port for development is 7810");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(index)
    })
    .bind(("0.0.0.0", 7810))?
    .run()
    .await
}