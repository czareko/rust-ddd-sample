use std::rc::Rc;
use crate::application::requests::createuser_request::CreateUserRequest;
use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;

pub struct CreateUserHandler{
    user_repository: Rc<dyn UserRepository>,
}

impl CreateUserHandler {
    pub fn new(user_repository: Rc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub fn execute(&self, request: CreateUserRequest) {
        let id = self.user_repository.next_identity();
        let user = User::new(
            id,
            request.name.as_str(),
            request.age,
            request.gender
        );

        self.user_repository.save(user);
    }
}

#[cfg(test)]
mod test{
    use std::rc::Rc;
    use fake::{Fake, Faker};
    use fake::faker::name::en::Name;

    use crate::application::handlers::createuser_handler::CreateUserHandler;
    use crate::application::requests::createuser_request::CreateUserRequest;
    use crate::domain::entities::user::User;
    use crate::domain::repositories::user_repository::MockUserRepository;


    #[test]
    fn should_execute_create_user_handler() {

        //when
        let id: i32 = Faker.fake();
        let mut user_repository_mock = MockUserRepository::new();

        //then
        user_repository_mock
            .expect_next_identity()
            .times(1)
            .return_const(id.clone());

        user_repository_mock
            .expect_save()
            .withf(move |user: &User| user.id.eq(&id))
            .times(1)
            .return_const(());

        let create_user_handler =
            CreateUserHandler::new(Rc::new(user_repository_mock));

        create_user_handler.execute(CreateUserRequest::new(
            Name().fake::<String>().as_str().to_string(),
            Faker.fake(),
            Faker.fake(),
        ));
    }
}