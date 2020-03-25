use std::fmt::{Debug, Display};
use serde::Serialize;
use crate::core::model;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Response {
    Success(SuccessResponse),
    Error(ErrorResponse),
}

impl Response {
    pub fn from_user(user: model::User, info: impl Into<String>) -> Self {
        Self::Success(
            SuccessResponse {
                info: info.into(),
                data: SuccessDataResponse::User { user: user.into() },
            }
        )
    }

    pub fn from_users(users: Vec<model::User>, info: impl Into<String>) -> Self {
        Self::Success(
            SuccessResponse {
                info: info.into(),
                data: SuccessDataResponse::Users { users: users.into_iter().map(Into::into).collect() },
            }
        )
    }

    pub fn from_error<E: Display + Debug>(err: E) -> Self {
        Self::Error(err.into())
    }
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    info: String,
    #[serde(flatten)]
    data: SuccessDataResponse,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum SuccessDataResponse {
    User { user: User },
    Users { users: Vec<User> },
}

#[derive(Debug, Serialize)]
pub struct User {
    id: u32,
    name: String,
}

impl From<model::User> for User {
    fn from(user: model::User) -> Self {
        Self {
            id: user.id.into(),
            name: user.name,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    kind: String,
    info: String,
}

impl ErrorResponse {
    pub fn new(kind: impl Into<String>, info: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            info: info.into(),
        }
    }
}

impl<T: Display + Debug> From<T> for ErrorResponse {
    fn from(err: T) -> Self {
        Self::new(format!("{:?}", err), format!("{}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_response() {
        let alice = model::User {
            id: model::UserId(1),
            name: "Alice".to_string(),
        };
        let response = Response::from_user(alice.clone(), "Processing info");
        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(r#"{"success":{"info":"Processing info","user":{"id":1,"name":"Alice"}}}"#, json);

        let bob = model::User {
            id: model::UserId(2),
            name:"Bob".to_string(),
        };
        let response = Response::from_users(vec![alice, bob], "Processing info");
        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(r#"{"success":{"info":"Processing info","users":[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]}}"#, json);
    }

    #[test]
    fn error_response() {
        let response = Response::Error(ErrorResponse::new("ErrorKind", "Error info"));
        let json = serde_json::to_string(&response).unwrap();
        assert_eq!(r#"{"error":{"kind":"ErrorKind","info":"Error info"}}"#, json);
    }
}