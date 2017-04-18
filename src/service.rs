use model::User;
use errors::*;
use diesel::prelude::*;
use diesel;
use diesel::pg::PgConnection;
use ring_pwhash::scrypt;

use util::{JsonResponse, Page};
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
        JsonResponse::from(users.filter(id.eq(user_id))
            .first::<User>(self.connection)
            .map_err(From::from))
    }

    pub fn create_user(&self, request: CreateUserRequest) -> JsonResponse<User, Error> {
        use schema::users;

        let params = scrypt::ScryptParams::new(14, 8, 2);

        let user = User {
            name: request.name,
            pw_hash: scrypt::scrypt_simple(&request.password, &params).unwrap(),
            id: 0,
        };
        let result = diesel::insert(&user)
            .into(users::table)
            .get_result::<User>(self.connection)
            .map_err(From::from);
        JsonResponse::from(result)
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
        use schema::posts;

        let result = diesel::insert(&request)
            .into(posts::table)
            .get_result::<Post>(self.connection)
            .map_err(From::from);
        JsonResponse::from(result)
    }

    pub fn find_one(&self, post_id: i32, user_id: i32) -> JsonResponse<Post, Error> {
        use schema::posts::dsl::*;
        let result = posts.filter(id.eq(post_id))
            .filter(owner_id.eq(user_id))
            .first(self.connection)
            .map_err(From::from);
        JsonResponse::from(result)
    }

    pub fn find_page(&self,
                     user_id: i32,
                     page_num: i64,
                     page_size: i64)
                     -> JsonResponse<Page<Post>, Error> {
        use schema::posts::dsl::*;

        let offset = page_num * page_size;
        let limit = offset + page_size;

        let result = posts.filter(owner_id.eq(user_id))
            .offset(offset)
            .limit(limit)
            .load(self.connection)
            .and_then(|v| Ok(Page::new(v, page_num, 0, page_size)))
            .map_err(From::from);
        JsonResponse::from(result)
    }

    pub fn update_post(&self, post: Post) -> JsonResponse<Post, Error> {
        JsonResponse::from(post.save_changes(self.connection).map_err(From::from))
    }
}
