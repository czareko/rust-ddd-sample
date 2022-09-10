use crate::domain::entities::gender_type::Gender;
use crate::domain::entities::user::User;

#[derive(Debug, PartialEq, Eq)]
#[readonly::make]
pub struct UserDto {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub gender: Gender,
}

impl UserDto {
    pub fn to_dto(user: &User) -> Self {
        Self {
            id: user.id,
            name: user.name.clone(),
            age: user.age,
            gender: user.gender.clone(),
        }
    }
}

#[derive(Debug)]
pub struct DtoList<T>(pub Vec<T>);

#[cfg(test)]
mod test {
    use crate::application::dto::user_dto::UserDto;
    use crate::domain::entities::gender_type::Gender;
    use crate::domain::entities::user::User;

    #[test]
    fn should_create_user_dto_from_entity() {
        //when
        let user = User::new(1, "Cezary", 89, Gender::Male);
        //given
        let user_dto = UserDto::to_dto(&user);
        //then
        assert_eq!(user.id, user_dto.id);
        assert_eq!(user.name, user_dto.name);
        assert_eq!(user.age, user_dto.age);
        assert_eq!(user.gender, user_dto.gender);
    }
}