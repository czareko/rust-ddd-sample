use crate::domain::entities::gender_type::Gender;
use fake::{Dummy, Fake};

#[derive(Clone, Dummy, PartialEq, Eq)]
pub struct User{
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub gender: Gender,
}

impl User {
    pub fn new(id: i32, name: &str, age: i32,gender: Gender) -> Self {
        Self { id, name: name.to_string(), age , gender}
    }

    pub fn update(&mut self,name: &str,age: i32 ,gender: Gender){
        self.name = name.to_string();
        self.age = age;
        self.gender = gender;
    }
}

#[cfg(test)]
mod test {
    use fake::{Fake, Faker};
    use crate::domain::entities::gender_type::Gender;
    use crate::domain::entities::user::User;

    #[test]
    fn create_user(){
        let id: i32 = Faker.fake();
        let name: String = Faker.fake();
        let age: i32 = Faker.fake();
        let gender: Gender = Faker.fake();

        let user = User::new(id,name.as_str(), age,gender.clone());

        assert_eq!(user.id,id);
        assert_eq!(user.name,name.as_str());
        assert_eq!(user.age,age);
        assert_eq!(user.gender,gender.clone());

    }
    #[test]
    fn update_user(){
        let mut user: User = Faker.fake();
        let id = user.id.clone();

        let new_name: String = Faker.fake();
        let new_age: i32 = Faker.fake();
        let new_gender: Gender = Faker.fake();

        assert_ne!(user.name, new_name);
        assert_ne!(user.age, new_age);

        user.update(new_name.as_str(), new_age,new_gender.clone());

        assert_eq!(user.id, id);
        assert_eq!(user.name, new_name);
        assert_eq!(user.gender, new_gender.clone());
    }
}