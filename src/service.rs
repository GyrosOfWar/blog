pub mod user {
    use model::User;
    use errors::*;
    use diesel::prelude::*;
    use diesel;
    use diesel::pg::PgConnection;
    use ring_pwhash::scrypt;

    use model::CreateUserRequest;

    const SCRYPT_LOG_N: u8 = 14;
    const SCRYPT_R: u32 = 8;
    const SCRYPT_P: u32 = 1;

    pub fn find_one(user_id: i32, conn: &PgConnection) -> Result<Option<User>> {
        use schema::users::dsl::*;
        users
            .filter(id.eq(user_id))
            .first::<User>(conn)
            .optional()
            .map_err(From::from)
    }

    pub fn create_user(request: CreateUserRequest, conn: &PgConnection) -> Result<User> {
        use schema::users;

        let params = scrypt::ScryptParams::new(SCRYPT_LOG_N, SCRYPT_R, SCRYPT_P);

        let user = User {
            name: request.name,
            pw_hash: scrypt::scrypt_simple(&request.password, &params).unwrap(),
            id: 0,
        };
        diesel::insert(&user)
            .into(users::table)
            .get_result::<User>(conn)
            .map_err(From::from)
    }

    pub fn find_by_id(user_id: i32, conn: &PgConnection) -> Result<User> {
        use schema::users::dsl::*;

        users
            .filter(id.eq(user_id))
            .first::<User>(conn)
            .map_err(From::from)
    }

    pub fn find_by_name(username: &str, conn: &PgConnection) -> Result<Option<User>> {
        use schema::users::dsl::*;

        users
            .filter(name.eq(username))
            .first::<User>(conn)
            .optional()
            .map_err(From::from)
    }

    pub fn get_name(user_id: i32, conn: &PgConnection) -> Result<String> {
        use schema::users;

        users::table
            .select(users::name)
            .filter(users::id.eq(user_id))
            .first(conn)
            .map_err(From::from)
    }
}

pub mod post {
    use errors::*;
    use diesel::prelude::*;
    use diesel;
    use diesel::pg::PgConnection;

    use util::Page;
    use model::{CreatePostRequest, Post};

    pub fn insert_post(request: CreatePostRequest, conn: &PgConnection) -> Result<Post> {
        use schema::posts;

        diesel::insert(&request)
            .into(posts::table)
            .get_result::<Post>(conn)
            .map_err(From::from)
    }

    pub fn find_one(post_id: i32, conn: &PgConnection) -> Result<Option<Post>> {
        use schema::posts::dsl::*;
        
        posts
            .filter(id.eq(post_id))
            .first(conn)
            .optional()
            .map_err(From::from)
    }

    pub fn find_page(user_id: i32,
                     page_num: i64,
                     page_size: i64,
                     conn: &PgConnection)
                     -> Result<Page<Post>> {
        use schema::posts::dsl::*;

        let offset = page_num * page_size;
        let limit = offset + page_size;

        posts
            .filter(owner_id.eq(user_id).and(published.eq(true)))
            .offset(offset)
            .limit(limit)
            .load(conn)
            .and_then(|v| Ok(Page::new(v, page_num, 0, page_size)))
            .map_err(From::from)
    }

    pub fn update_post(post: &Post, conn: &PgConnection) -> Result<Post> {
        post.save_changes(conn).map_err(From::from)
    }

    pub fn get_by_tag(user_id: i32, tag: &str, conn: &PgConnection) -> Result<Vec<Post>> {
        use schema::posts::dsl::*;

        posts
            .filter(owner_id.eq(user_id).and(tags.contains(vec![tag])))
            .load(conn)
            .map_err(From::from)
    }
}
