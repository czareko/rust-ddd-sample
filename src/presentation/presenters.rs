use std::fmt::{Display, Formatter};
use crate::application::dto::user_dto::{DtoList, UserDto};

impl Display for UserDto {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User #{}: {}, age {}, gender: {}",
            self.id, self.name, self.age, self.gender
        )
    }
}

impl Display for DtoList<UserDto> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return write!(f, "No users");
        }

        let mut to_display = String::new();
        to_display.push_str("User list\n");
        to_display.push_str("----------------------------------------\n\n");

        for user in &self.0 {
            to_display.push_str(format!("{}\n", user).as_str());
        }

        write!(f, "{}", to_display)
    }
}


#[cfg(test)]
mod test{
    use fake::{Fake, Faker};
    use crate::application::dto::user_dto::{DtoList, UserDto};
    use crate::domain::entities::gender_type::Gender;
    use crate::domain::entities::user::User;

    #[test]
    fn should_display_user_dto() {
        //when
        let user = User::new(1, "Cezary",10,Gender::Male);
        let user_dto = UserDto::to_dto(&user);
        //then
        assert_eq!(user_dto.to_string(), "User #1: Cezary, age 10, gender: male");

    }

    #[test]
    fn should_display_empty_user_dto_list() {
        //when
        let user_dto_list: DtoList<UserDto> = DtoList(Vec::new());
        //then
        assert_eq!(user_dto_list.to_string(), "No users")
    }

    #[test]
    fn should_display_user_dto_list_with_items() {
        //when
        let user_a: User = Faker.fake();
        let user_b: User = Faker.fake();
        //given
        let dto_a = UserDto::to_dto(&user_a);
        let dto_b = UserDto::to_dto(&user_b);

        let user_dto_list: DtoList<UserDto> = DtoList(vec![dto_a, dto_b]);

        let mut expected = "User list
----------------------------------------

"
            .to_string();

        for dto in &user_dto_list.0 {
            expected.push_str(dto.to_string().as_str());
            expected.push('\n');
        }
        //then
        assert_eq!(user_dto_list.to_string(), expected)
    }
}