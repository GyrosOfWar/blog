use std::io::Read;
use std::fmt::Debug;
use std::cmp;

use iron::prelude::*;
use iron::status;
use iron::status::Status;
use iron_diesel_middleware::DieselReqExt;
use serde::Deserialize;

use service::{UserService, PostService};
use errors::*;
use auth::{UserCredentials, JwtToken};
use model::{CreateUserRequest, CreatePostRequest, Post};
use serde_json;
use util::{JsonResponse, markdown_to_html, MarkdownMode};
use req_ext::*;

const MAX_QUERY_LEN: i64 = 50;

macro_rules! jtry {
    ($result:expr) => (jtry!($result, status::BadRequest));

    ($result:expr, $err_status:expr) => (match $result {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(why) => {
            let resp: JsonResponse<(), _> = JsonResponse::Error(Error::from(why));
            return resp.into_iron_result(status::Ok, $err_status)
        }
    })
}

macro_rules! jexpect {
    ($option:expr) => (jexpect!($option, status::BadRequest));

    ($option:expr, $err_status:expr) => (match $option {
        ::std::option::Option::Some(val) => val,
        ::std::option::Option::None => {
            let resp: JsonResponse<(), _> = JsonResponse::Error(Error::Other(String::from("Internal server error")));
            return resp.into_iron_result(status::Ok, $err_status)
        }
    })
}

fn read_json_body<T>(req: &mut Request) -> Result<T>
    where T: Deserialize + Debug
{
    let mut body = String::new();
    try!(req.body.read_to_string(&mut body));
    debug!("Request body: {}", body);
    let res = serde_json::from_str(&body).map_err(From::from);
    debug!("Result: {:?}", res);
    res
}

fn error(msg: &str, status: Status) -> IronResult<Response> {
    JsonResponse::Error::<(), _>(Error::Other(String::from(msg)))
        .into_iron_result(status::Created, status)
}

pub struct UserController;

impl UserController {
    pub fn make_jwt_token(req: &mut Request) -> IronResult<Response> {
        let conn = req.db_conn();
        let service = UserService::new(&*conn);
        match read_json_body::<UserCredentials>(req) {
            Ok(creds) => service.make_token(&creds.name, &creds.password, super::SECRET).into(),
            Err(why) => error(&format!("{}", why), status::BadRequest),
        }
    }

    pub fn create_user(req: &mut Request) -> IronResult<Response> {
        let conn = req.db_conn();
        let service = UserService::new(&*conn);

        match read_json_body::<CreateUserRequest>(req) {
            Ok(req) => {
                service.create_user(req).into_iron_result(status::Created, status::BadRequest)
            }
            Err(why) => error(&format!("{}", why), status::BadRequest),
        }
    }

    pub fn get_user(req: &mut Request) -> IronResult<Response> {
        let user_id = jexpect!(req.path_param("user_id"));
        let token = jexpect!(req.extensions.get::<JwtToken>());
        if token.is_authenticated(user_id) {
            let conn = req.db_conn();
            let service = UserService::new(&*conn);
            service.find_one(user_id).into()
        } else {
            error("Invalid credentials", status::Unauthorized)
        }
    }

    pub fn edit_post(req: &mut Request) -> IronResult<Response> {
        let user_id: i32 = jexpect!(req.path_param("user_id"));
        let post_id: i32 = jexpect!(req.path_param("post_id"));
        let post: Post = jtry!(read_json_body(req));
        let conn = req.db_conn();
        let token = jexpect!(req.extensions.get::<JwtToken>());
        if token.is_authenticated(user_id) {
            if post_id != post.id || user_id != post.owner_id {
                error("Wrong owner or post id!", status::Forbidden)
            } else {
                let service = PostService::new(&*conn);
                service.update_post(post).into()
            }
        } else {
            error("Invalid credentials", status::Unauthorized)
        }
    }
}

pub struct PostController;

impl PostController {
    pub fn get_post(req: &mut Request) -> IronResult<Response> {
        let conn = req.db_conn();
        let service = PostService::new(&*conn);
        let post_id = jexpect!(req.path_param("post_id"));
        let user_id = jexpect!(req.path_param("user_id"));
        debug!("User ID = {}, post ID = {}", user_id, post_id);
        service.find_one(post_id, user_id).into_iron_result(status::Ok, status::BadRequest)
    }

    pub fn add_post(req: &mut Request) -> IronResult<Response> {
        let mut create_request: CreatePostRequest = jtry!(read_json_body(req));
        // TODO read config for this
        create_request.content = markdown_to_html(&create_request.content, MarkdownMode::Plain);
        let user_id = jexpect!(req.path_param("user_id"));
        let token = jexpect!(req.extensions.get::<JwtToken>());
        if token.is_authenticated(user_id) {
            let conn = req.db_conn();
            let service = PostService::new(&*conn);
            service.insert_post(create_request)
                .into_iron_result(status::Created, status::InternalServerError)
        } else {
            error("Invalid credentials", status::Unauthorized)
        }
    }

    pub fn get_posts(req: &mut Request) -> IronResult<Response> {
        let user_id: i32 = iexpect!(req.path_param("user_id"));
        let conn = req.db_conn();
        let page_size = req.url_param("page_size").unwrap_or(25);
        let page = req.url_param("page").unwrap_or(0);
        debug!("Page: {}, Page size: {}", page, page_size);
        let page_size = cmp::min(page_size, MAX_QUERY_LEN);
        let service = PostService::new(&*conn);
        service.find_page(user_id, page, page_size).into()
    }
}
