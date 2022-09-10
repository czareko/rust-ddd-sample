use std::rc::Rc;
use crate::application::dto::user_dto::UserDto;
use crate::application::requests::getuser_request::GetUserRequest;
use crate::domain::repositories::user_repository::UserRepository;

pub struct GetUserHandler {
    user_repository: Rc<dyn UserRepository>,
}

impl GetUserHandler {
    pub fn new(user_repository: Rc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub fn execute(&self, request: GetUserRequest) -> Result<UserDto, String> {
        let user = self.user_repository.by_id(&request.id)?;

        Ok(UserDto::to_dto(&user))
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use fake::{Fake, Faker};
    use fake::faker::lorem::en::Sentence;
    use crate::application::dto::user_dto::UserDto;
    use crate::application::handlers::getuser_handler::GetUserHandler;
    use crate::application::requests::getuser_request::GetUserRequest;
    use crate::domain::entities::user::User;
    use crate::domain::repositories::user_repository::MockUserRepository;

    #[test]
    fn should_execute_get_user_handler_ok() {
        //when
        let user: User = Faker.fake();
        let user_expectation = UserDto::to_dto(&user);
        let id = user.id;
        let id2 = user.id;
        let mut user_repository_mock = MockUserRepository::new();
        //given
        user_repository_mock
            .expect_by_id()
            .withf(move |x: &i32| x.eq(&id))
            .times(1)
            .return_const(Ok(user));

        let get_user_handler =
            GetUserHandler::new(Rc::new(user_repository_mock));
        let result =
            get_user_handler.execute(GetUserRequest::new(id2));
        //then
        assert_eq!(result, Ok(user_expectation))
    }

    #[test]
    fn should_execute_get_user_handler_err() {
        let id: i32 = Faker.fake();
        let id2 = id.clone();
        let mut user_repository_mock = MockUserRepository::new();

        let error_txt: String = Sentence(3..8).fake();

        user_repository_mock
            .expect_by_id()
            .withf(move |x: &i32| x.eq(&id))
            .times(1)
            .return_const(Err(error_txt.clone()));

        let get_user_handler =
            GetUserHandler::new(Rc::new(user_repository_mock));

        let result =
            get_user_handler.execute(GetUserRequest::new(id2));

        assert_eq!(result, Err(error_txt))
    }
}