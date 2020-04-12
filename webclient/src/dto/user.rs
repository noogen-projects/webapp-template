#[derive(Clone, PartialEq, PartialOrd)]
pub struct User {
    pub id: String,
    pub name: String,
}

impl User {
    pub fn new(id: impl ToString, name: impl ToString) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
        }
    }
}