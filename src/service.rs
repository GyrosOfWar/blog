use model::User;
use errors::*;
use diesel::prelude::*;
use diesel;
use diesel::pg::PgConnection;
use pwhash::bcrypt;

use auth::TokenMaker;
use util::JsonResponse;
use model::{CreateUserRequest, CreatePostRequest, Post};

pub struct UserService<'a> {
    connection: &'a PgConnection,
}

impl<'a> UserService<'a> {
    pub fn new(connection: &'a PgConnection) -> UserService<'a> {
        UserService { connection: connection }
    }

    pub fn find_one(&self, user_id: i32) -> JsonResponse<User, Error> {
        use schema::users::dsl::*;
        JsonResponse::from_result(users.filter(id.eq(user_id))
            .first::<User>(self.connection)
            .map_err(From::from))
    }

    pub fn make_token(&self,
                      username: &str,
                      password: &str,
                      server_secret: &[u8])
                      -> JsonResponse<String, Error> {
        use schema::users::dsl::*;
        let result = users.filter(name.eq(username))
            .first::<User>(self.connection)
            .map_err(Error::from)
            .and_then(|user| {
                if bcrypt::verify(password, &user.pw_hash) {
                    let token_maker = TokenMaker::new(server_secret);
                    let user_id = format!("{}", user.id);
                    token_maker.make_token(&user_id).ok_or(Error::CreateToken)
                } else {
                    Err(Error::InvalidCredentials)
                }
            });
        JsonResponse::from_result(result)
    }

    pub fn create_user(&self, request: CreateUserRequest) -> JsonResponse<User, Error> {
        use schema::users;

        let user = User {
            name: request.name,
            pw_hash: bcrypt::hash(&request.password).unwrap(),
            id: 0,
        };
        let result = diesel::insert(&user)
            .into(users::table)
            .get_result::<User>(self.connection)
            .map_err(From::from);
        JsonResponse::from_result(result)
    }
}

pub struct PostService<'a> {
    connection: &'a PgConnection,
}

impl<'a> PostService<'a> {
    pub fn new(connection: &'a PgConnection) -> PostService<'a> {
        PostService { connection: connection }
    }

    pub fn insert_post(&self, request: CreatePostRequest) -> JsonResponse<Post, Error> {
        unimplemented!()
    }
}
