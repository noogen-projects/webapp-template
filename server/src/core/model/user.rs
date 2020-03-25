#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct UserId(pub u32);

impl UserId {
    pub fn is_exist(&self) -> bool {
        self.0 != u32::default()
    }
}

impl From<UserId> for u32 {
    fn from(user_id: UserId) -> Self {
        user_id.0
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: String,
}