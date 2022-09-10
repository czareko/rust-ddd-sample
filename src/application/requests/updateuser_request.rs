use crate::domain::entities::gender_type::Gender;

#[readonly::make]
pub struct UpdateUserRequest {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub gender: Gender,
}

impl UpdateUserRequest {
    pub fn new(id: i32, name: String, age: i32, gender: Gender) -> Self {
        Self { id, name, age, gender }
    }
}