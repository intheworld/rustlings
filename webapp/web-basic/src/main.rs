use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::get;
use std::sync::Mutex;

struct AppState {
    app_name: String
}

struct AppStateWithCounter {
    counter: Mutex<i32>
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("Hello {}!", app_name)
}

async fn _index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {}", counter)
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(||HttpResponse::Ok().body("test")))
            .route(web::head().to(||HttpResponse::MethodNotAllowed()))
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(||  HttpResponse::Ok().body("app")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
    );
}


#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    println!("starting server !");
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0)
    });
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web")
            })
            .app_data(counter.clone())
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .route("/", web::get().to(_index))
            .service(web::scope("/app"))
            .route("/index.html", web::get().to(index))
    }).bind("127.0.0.1:8088")?
        .run()
        .await
}
