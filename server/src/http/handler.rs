use actix_web::{get, post, web, Responder};
use crate::{
    dao::user::{UserDao, ConnectionsPool},
    http::dto::Response,
    core::{
        service::UserService,
        model::{User, UserId},
    },
};

#[post("/user")]
pub async fn add_user(pool: web::Data<ConnectionsPool>, name: web::Json<String>) -> impl Responder {
    let user_dao = UserDao::new(pool.get_ref().clone());
    let user = User { id: UserId::default(), name: name.into_inner() };

    let response = match web::block(move || user_dao.save(user)).await {
        Ok(user) => Response::from_user(user, "User was saved"),
        Err(err) => Response::from_error(err),
    };
    web::Json(response)
}

#[get("/user/{id}")]
pub async fn get_user(pool: web::Data<ConnectionsPool>, id: web::Path<u32>) -> impl Responder {
    let user_dao = UserDao::new(pool.get_ref().clone());
    let user_id = UserId(id.into_inner());

    let response = match web::block(move || user_dao.get_by_id(user_id)).await {
        Ok(user) => Response::from_user(user, "User exists"),
        Err(err) => Response::from_error(err),
    };
    web::Json(response)
}

#[post("/user/{id}")]
pub async fn edit_user(pool: web::Data<ConnectionsPool>, id: web::Path<u32>, name: web::Json<String>) -> impl Responder {
    let user_dao = UserDao::new(pool.get_ref().clone());
    let user = User { id: UserId(id.into_inner()), name: name.into_inner() };

    let response = match web::block(move || user_dao.save(user)).await {
        Ok(user) => Response::from_user(user, "User was updated"),
        Err(err) => Response::from_error(err),
    };
    web::Json(response)
}

#[get("/users")]
pub async fn list_users(pool: web::Data<ConnectionsPool>) -> impl Responder {
    let user_dao = UserDao::new(pool.get_ref().clone());

    let response = match web::block(move || user_dao.list()).await {
        Ok(users) => Response::from_users(users, "Users exist"),
        Err(err) => Response::from_error(err),
    };
    web::Json(response)
}