extern crate actix_web;
extern crate actix_files;
extern crate dotenv;
extern crate toml;

use actix_web::{error, middleware, App, HttpRequest, HttpResponse, HttpServer};
use actix_files::Files;
use tera::Tera;

mod routes;
mod mntconfig;

use crate::routes::index::*;
use crate::mntconfig::Config;

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    use actix_web::error::JsonPayloadError;

    let detail = err.to_string();
    let resp = match &err {
        JsonPayloadError::ContentType => {
            HttpResponse::UnsupportedMediaType().body(detail)
        }
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().body(detail)
        }
        _ => HttpResponse::BadRequest().body(detail),
    };
    error::InternalError::from_response(err, resp).into()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let config_str = std::fs::read_to_string("mntconfig.toml").unwrap();
    let mntconfig:Config = toml::from_str(&config_str).unwrap();

    let bind = mntconfig.bind.clone();

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        let tera =
            Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
        
        App::new()
            // tera templating
            .data(tera)
            .data(mntconfig.clone())
            .wrap(middleware::Logger::default())
            .service(get_index)
            .service(put_lights_json)
            .service(Files::new("/css", "static/css/"))
            .service(Files::new("/js", "static/js/"))
            .service(Files::new("/img", "static/img/"))
    })
    .bind(&bind)?
    .run()
    .await
}
