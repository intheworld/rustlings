use actix_web::{web, App, HttpResponse, HttpServer, Result, middleware};
use std::io::Write;
use std::io::Read;
use std::fs::File;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use log::{debug, Level};
use actix_web::middleware::Logger;

use crate::paste_id::PasteID;

mod paste_id;

const HOST: &str = "127.0.0.1:8080";
const ID_LENGTH: usize = 8;

struct AppState {
    app_name: String
}

#[derive(Deserialize, Serialize)]
struct FormData {
    username: String,
    content: String
}

async fn upload(paste: web::Form<FormData>) -> Result<HttpResponse> {
    debug!("this is a debug {}", paste.username);
    let id = PasteID::new(ID_LENGTH);
    let filename = format!("upload/{id}", id = id);
    let url = format!("http://{host}/{id}", host = HOST, id = id);
    let mut buffer = File::create(&filename)?;
    buffer.write_all(paste.content.as_bytes())?;
    Ok(actix_web::HttpResponse::SeeOther()
        .header(actix_web::http::header::LOCATION, url)
        .finish())
}

async fn retrieve(info: web::Path<String>) -> Result<HttpResponse> {
    debug!("this is a debug {}", info);
    let filename = format!("upload/{id}", id = info);
    let file = File::open(&filename);
    if file.is_ok() {
        let mut buffer = String::new();
        file.unwrap().read_to_string(&mut buffer);
        Ok(HttpResponse::from(buffer))
    } else {
        Ok(HttpResponse::build(StatusCode::NOT_FOUND).finish())
    }
}

async fn index() -> Result<HttpResponse> {
    debug!("this is a index debug");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/upload").route(web::post().to(upload)))
            .service(web::resource("/{id}").route(web::get().to(retrieve))),
    );
}

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    println!("starting server!");
    std::env::set_var("RUST_LOG", "actix_web=trace");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(app_config)
    }).bind(HOST)?
        .run()
        .await
}
