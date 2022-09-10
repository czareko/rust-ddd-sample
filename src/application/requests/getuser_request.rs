pub struct GetUserRequest{
    id: i32,
}

impl GetUserRequest {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}