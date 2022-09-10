use crate::domain::entities::gender_type::Gender;

pub struct CreateUserRequest{
    name: String,
    age: i32,
    gender: Gender,
}

impl CreateUserRequest {
    pub fn new(name: String, age: i32, gender: Gender) -> Self {
        Self { name, age, gender }
    }
}

