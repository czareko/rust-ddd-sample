use std::rc::Rc;
use crate::application::requests::updateuser_request::UpdateUserRequest;
use crate::domain::repositories::user_repository::UserRepository;

pub struct UpdateUserHandler{
    user_repository: Rc<dyn UserRepository>,
}

impl UpdateUserHandler {
    pub fn new(user_repository: Rc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub fn execute(&self, request: UpdateUserRequest) -> Result<(), String> {
        let mut user = self.user_repository.by_id(&request.id)?;
        user.update(request.name.as_str(), request.age,request.gender);

        self.user_repository.save(user);

        Ok(())
    }
}

#[cfg(test)]
mod test{
    use std::rc::Rc;
    use fake::{Fake, Faker};
    use fake::faker::lorem::en::Sentence;
    use crate::application::handlers::updateuser_handler::UpdateUserHandler;
    use crate::application::requests::updateuser_request::UpdateUserRequest;
    use crate::domain::entities::gender_type::Gender;
    use crate::domain::entities::user::User;
    use crate::domain::repositories::user_repository::MockUserRepository;

    #[test]
    fn should_execute_update_user_handler_ok() {
        //when
        let user: User = Faker.fake();

        let id = user.id;
        let id2 = id.clone();
        let id3 = id.clone();

        let new_name: String = Faker.fake();
        let new_age: i32 = Faker.fake();
        let new_gender: Gender = Faker.fake();

        let expected_user = User::new(id2, new_name.as_str(), new_age, new_gender);

        let mut user_repository_mock = MockUserRepository::new();

        //given

        user_repository_mock
            .expect_by_id()
            .withf(move |x: &i32| x.eq(&id))
            .times(1)
            .return_const(Ok(user));

        user_repository_mock
            .expect_save()
            .withf(move |x: &User| x == &expected_user)
            .times(1)
            .return_const(());

        let update_user_handler =
            UpdateUserHandler::new(Rc::new(user_repository_mock));

        //then
        let result = update_user_handler.execute(UpdateUserRequest::new(
            id3,new_name,new_age,new_gender
        ));

        assert_eq!(Ok(()), result);
    }

    #[test]
    fn edit_client_use_case_handler_execute_on_non_existing_client() {
        let user: User = Faker.fake();

        let id = user.id;
        let id3 = id.clone();

        let new_name: String = Faker.fake();
        let new_age: i32 = Faker.fake();
        let new_gender: Gender = Faker.fake();

        let error_txt: String = Sentence(3..8).fake();

        let mut user_repository_mock = MockUserRepository::new();

        user_repository_mock
            .expect_by_id()
            .withf(move |x: &i32| x.eq(&id))
            .times(1)
            .return_const(Err(error_txt.clone()));

        user_repository_mock
            .expect_save()
            .times(0)
            .return_const(());

        let update_user_handler =
            UpdateUserHandler::new(Rc::new(user_repository_mock));

        let result = update_user_handler.execute(UpdateUserRequest::new(
            id3,new_name,new_age,new_gender));

        assert_eq!(Err(error_txt), result);
    }
}