#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use dao::user::{ConnectionsPool};
use diesel::r2d2::{Pool, ConnectionManager};
use http::handler;
use crate::settings::Settings;

mod dao;
mod core;
mod http;
mod settings;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let settings = Settings::new().expect("Error read settings");
    let manager = ConnectionManager::new(&settings.database_url);
    let pool: ConnectionsPool = Pool::new(manager).expect("Error creating connection pool");

    HttpServer::new(move || App::new()
        .data(pool.clone())
        .service(handler::index)
        .service(handler::get_user)
        .service(handler::add_user)
        .service(handler::edit_user)
        .service(handler::list_users)
    )
        .bind(&settings.app_address)?
        .run()
        .await
}
