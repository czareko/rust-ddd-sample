use crate::domain::entities::gender_type::Gender;

pub struct UpdateUserRequest{
    id: i32,
    name: String,
    age: i32,
    gender: Gender,
}

impl UpdateUserRequest {
    pub fn new(id: i32, name: String, age: i32, gender: Gender) -> Self {
        Self { id, name, age, gender }
    }
}