use crate::domain::entities::gender_type::Gender;

pub struct User{
    id: i32,
    name: String,
    age: i32,
    gender: Gender,
}

impl User {
    pub fn new(id: i32, name: &str, age: i32,gender: Gender) -> Self {
        Self { id, name: name.to_string(), age , gender}
    }

    pub fn update(&mut self,name: &str,age: age,gender: Gender){
        self.name = name.to_string();
        self.age = age;
        self.gender = gender;
    }

}

