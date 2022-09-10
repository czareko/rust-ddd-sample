use crate::domain::entities::gender_type::Gender;

#[readonly::make]
pub struct CreateUserRequest{
    pub name: String,
    pub age: i32,
    pub gender: Gender,
}

impl CreateUserRequest {
    pub fn new(name: String, age: i32, gender: Gender) -> Self {
        Self { name, age, gender }
    }
}

