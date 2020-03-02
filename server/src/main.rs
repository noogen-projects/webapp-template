#[macro_use]
extern crate diesel;

use actix_web::{get, post, web, App, HttpServer, Responder};
use dao::user::{UserDao, ConnectionsPool};
use diesel::r2d2::{Pool, ConnectionManager};
use crate::core::service::UserService;
use crate::core::model::{User, UserId};

mod dao;
mod core;

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let manager = ConnectionManager::new("mysql://webapp:webapp@localhost/webapp");
    let pool: ConnectionsPool = Pool::new(manager).expect("Error creating connection pool");

    HttpServer::new(move || App::new()
        .data(pool.clone())
        .service(index)
        .service(get_user)
        .service(add_user)
        .service(list_users)
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[post("/user")]
async fn add_user(pool: web::Data<ConnectionsPool>, name: web::Json<String>) -> impl Responder {
    let user_dao = UserDao::new(pool.get_ref().clone());
    let user = User { id: UserId::default(), name: name.into_inner() };

    match web::block(move || user_dao.save(user)).await {
        Ok(user) => format!("User was saved! User: {:#?}", user),
        Err(err) => format!("User was not save. Error: {:?}", err),
    }
}

#[get("/user/{id}")]
async fn get_user(pool: web::Data<ConnectionsPool>, id: web::Path<u32>) -> impl Responder {
    let user_dao = UserDao::new(pool.get_ref().clone());
    let user_id = UserId(id.into_inner());

    match web::block(move || user_dao.get_by_id(user_id)).await {
        Ok(user) => format!("User: {:#?}", user),
        Err(err) => format!("User does not exist. Error: {:?}", err),
    }
}

#[get("/users")]
async fn list_users(pool: web::Data<ConnectionsPool>) -> impl Responder {
    let user_dao = UserDao::new(pool.get_ref().clone());

    match web::block(move || user_dao.list()).await {
        Ok(users) => format!("Users list: {:#?}", users),
        Err(err) => format!("User does not exist. Error: {:?}", err),
    }
}
