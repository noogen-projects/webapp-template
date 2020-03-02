#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct UserId(pub u32);

#[derive(Debug)]
pub struct User {
    pub id: UserId,
    pub name: String,
}