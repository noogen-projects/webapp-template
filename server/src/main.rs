#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use dao::user::{ConnectionsPool};
use diesel::r2d2::{Pool, ConnectionManager};
use http::handler;

mod dao;
mod core;
mod http;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let manager = ConnectionManager::new("mysql://webapp:webapp@localhost/webapp");
    let pool: ConnectionsPool = Pool::new(manager).expect("Error creating connection pool");

    HttpServer::new(move || App::new()
        .data(pool.clone())
        .service(handler::index)
        .service(handler::get_user)
        .service(handler::add_user)
        .service(handler::edit_user)
        .service(handler::list_users)
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
