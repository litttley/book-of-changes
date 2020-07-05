#[macro_use]
extern crate diesel;
extern crate dotenv;
use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;




mod controller;
mod config;
use controller::controller_search;
use crate::config::init_db;
use crate::config::init_db::{MysqlPool};



type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[get("/resource1/{name}/index.html")]
async fn index(req: HttpRequest, name: web::Path<String>) -> String {
    println!("REQ: {:?}", req);
    format!("Hello: {}!\r\n", name)
}

async fn index_async(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!\r\n"
}

#[get("/")]
async fn no_params() -> &'static str {
    "Hello world!\r\n"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    /*let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");*/

  let mysqlPool   =  init_db::connect();
    HttpServer::new(move || {
        App::new()
            .data(mysqlPool.clone())
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(index)
            .service(no_params)
            .service(
                web::resource("/resource2/index.html")
                    .wrap(
                        middleware::DefaultHeaders::new().header("X-Version-R2", "0.3"),
                    )
                    .default_service(
                        web::route().to(|| HttpResponse::MethodNotAllowed()),
                    )
                    .route(web::get().to(index_async)),
            )
            .service(web::resource("/test1.html").to(|| async { "Test\r\n" }))
           .service(controller_search::no_params)
    })
        .bind("127.0.0.1:8080")?
        .workers(1)
        .run()
        .await
}
