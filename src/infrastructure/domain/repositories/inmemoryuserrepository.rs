use std::cell::RefCell;
use std::collections::HashMap;
use crate::domain::entities::gender_type::Gender;
use crate::domain::entities::user::User;
use crate::domain::repositories::userrepository::UserRepository;

pub struct InMemoryUserRepository {
    users: RefCell<HashMap<i32, User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        let users: HashMap<i32, User> = HashMap::new();

        Self {
            users: RefCell::new(users),
        }
    }
    pub fn new_with_samples() -> Self {
        let repository = Self::new();

        let user_a = User::new(1, "Zack", 25, Gender::Male);
        let user_b = User::new(2, "Georgia", 33, Gender::Female);

        repository.save(user_a);
        repository.save(user_b);

        repository
    }
}

impl UserRepository for InMemoryUserRepository {
    fn by_id(&self, id: &i32) -> Result<User, String> {
        let user = self.users.borrow().get(id).cloned();

        match user {
            Some(c) => Ok(c),
            None => Err(String::from("User not found for given ID")),
        }
    }

    fn save(&self, user: User) {
        self.users.borrow_mut().insert(user.id.clone(), user);
    }

    fn next_identity(&self) -> String {
        let size = self.users.borrow().len() + 1;

        size.to_string()
    }

    fn all(&self) -> Vec<User> {
        let mut result = Vec::new();

        for value in self.users.borrow().values() {
            result.push(value.clone())
        }

        result
    }
}