use rocket::{Outcome, State};
use rocket::request::{self, FromRequest, Request};

use model::User;
use service::user;
use db_util::Pool;

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let pool = match <State<Pool> as FromRequest>::from_request(request) {
            Outcome::Success(pool) => pool,
            Outcome::Failure(e) => return Outcome::Failure(e),
            Outcome::Forward(_) => return Outcome::Forward(()),
        };
        let connection = pool.get().unwrap();
        let id: Option<i32> = request
            .session()
            .get("user_id")
            .and_then(|cookie| cookie.value().parse().ok());
        let user = id.and_then(|id| user::find_by_id(id, &connection).ok());
        match user {
            Some(user) => Outcome::Success(user),
            None => Outcome::Forward(()),
        }
    }
}
