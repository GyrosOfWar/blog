use model::User;

#[derive(Debug)]
pub struct UserService;

impl UserService {
    pub fn find_one(&self, user_id: u32) -> Option<User> {
        None
    }
}