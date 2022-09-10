use crate::domain::entities::user::User;
use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait UserRepository {
    fn by_id(&self, id: &i32) -> Result<User, String>;
    fn save(&self, user: User);
    fn next_identity(&self) -> String;
    fn all(&self) -> Vec<User>;
}