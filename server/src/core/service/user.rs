use crate::core::model::{UserId, User};

#[derive(Debug)]
pub struct SavingUserError(pub String, pub User);

#[derive(Debug)]
pub enum GettingUserError {
    DbError(String),
    UserWasNotFound,
}

pub trait UserService {
    fn get_by_id(&self, id: UserId) -> Result<User, GettingUserError>;
    fn list(&self) -> Result<Vec<User>, GettingUserError>;
    fn save(&self, user: User) -> Result<User, SavingUserError>;
}