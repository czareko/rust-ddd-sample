use std::cell::RefCell;
use std::collections::HashMap;
use crate::domain::entities::gender_type::Gender;
use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;

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

    fn next_identity(&self) -> i32 {
        let size = self.users.borrow().len() + 1;
        size as i32
    }

    fn all(&self) -> Vec<User> {
        let mut result = Vec::new();

        for value in self.users.borrow().values() {
            result.push(value.clone())
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::domain::entities::gender_type::Gender;
    use crate::domain::entities::user::User;
    use crate::domain::repositories::user_repository::UserRepository;
    use crate::infrastructure::domain::repositories::inmemoryuser_repository::InMemoryUserRepository;

    #[test]
    fn should_create_new_repository() {
        //when
        let user_repository = InMemoryUserRepository::new();
        //given
        let user = User::new(1, "Alan", 22, Gender::Male);
        user_repository.save(user);
        //then
        assert_eq!(1, user_repository.all().len());
    }

    #[test]
    fn should_save_and_find_by_id() {
        //when
        let user_repository = InMemoryUserRepository::new();
        //given
        let user = User::new(1, "Alan", 22, Gender::Male);
        user_repository.save(user.clone());
        //then
        let user_a = user_repository.by_id(&1).unwrap();
        assert_eq!(user.id, user_a.id);
        assert_eq!(user.name, user_a.name);
        assert_eq!(user.age, user_a.age);
        assert_eq!(user.gender, user_a.gender)
    }

    #[test]
    fn should_get_all_users() {
        //when
        let user_repository = InMemoryUserRepository::new();
        //given
        let user_a = User::new(1, "Alan", 22, Gender::Male);
        let user_b = User::new(2, "Monika", 15, Gender::Female);
        user_repository.save(user_a);
        user_repository.save(user_b);
        //then
        let users = user_repository.all();

        assert_eq!(2, users.len());
    }

    #[test]
    fn should_return_next_id() {
        //when
        let user_repository = InMemoryUserRepository::new();
        //given
        let user_a = User::new(user_repository.next_identity(), "Alan", 22, Gender::Male);
        user_repository.save(user_a);
        let user_b = User::new(user_repository.next_identity(), "Monika", 15, Gender::Female);
        user_repository.save(user_b);

        //then
        assert_eq!(3, user_repository.next_identity());
    }

    #[test]
    fn should_create_repository_with_samples() {
        //when
        let user_repository = InMemoryUserRepository::new_with_samples();
        //then
        assert_eq!(2, user_repository.all().len());
    }
}