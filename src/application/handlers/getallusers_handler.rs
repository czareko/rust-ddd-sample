use std::rc::Rc;
use crate::application::dto::user_dto::{DtoList, UserDto};
use crate::domain::repositories::user_repository::UserRepository;

pub struct GetAllUsersHandler{
    user_repository: Rc<dyn UserRepository>,
}

impl GetAllUsersHandler {
    pub fn new(user_repository: Rc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub fn execute(&self) -> DtoList<UserDto> {
        DtoList(
            self.user_repository
                .all()
                .iter()
                .map(UserDto::to_dto)
                .collect(),
        )
    }
}