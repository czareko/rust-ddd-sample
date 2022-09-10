#[readonly::make]
pub struct GetUserRequest{
    pub id: i32,
}

impl GetUserRequest {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}