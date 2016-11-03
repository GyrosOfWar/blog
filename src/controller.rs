use std::io::Read;

use iron::prelude::*;
use iron::status;
use iron_diesel_middleware::DieselReqExt;
use serde::Deserialize;
use router::Router;

use service::{UserService, PostService};
use errors::*;
use auth::{UserCredentials, JwtToken};
use model::{CreateUserRequest, CreatePostRequest};
use serde_json;
use util::JsonResponse;

macro_rules! jtry {
    ($result:expr) => (jtry!($result, status::BadRequest));

    ($result:expr, $err_status:expr) => (match $result {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(why) => {
            let resp: JsonResponse<(), _> = JsonResponse::Error(why);
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
    where T: Deserialize
{
    let mut body = String::new();
    try!(req.body.read_to_string(&mut body));
    debug!("Request body: {}", body);
    serde_json::from_str(&body).map_err(From::from)
}

pub struct UserController;

impl UserController {
    pub fn make_jwt_token(req: &mut Request) -> IronResult<Response> {
        let conn = req.db_conn();
        let service = UserService::new(&*conn);
        match read_json_body::<UserCredentials>(req) {
            Ok(creds) => {
                service.make_token(&creds.name, &creds.password, super::SECRET)
                    .into_iron_result(status::Ok, status::BadRequest)
            }
            Err(why) => {
                JsonResponse::Error::<(), _>(why)
                    .into_iron_result(status::Created, status::BadRequest)
            }
        }
    }


    pub fn create_user(req: &mut Request) -> IronResult<Response> {
        let conn = req.db_conn();
        let service = UserService::new(&*conn);

        match read_json_body::<CreateUserRequest>(req) {
            Ok(req) => {
                service.create_user(req).into_iron_result(status::Created, status::BadRequest)
            }
            Err(why) => {
                JsonResponse::Error::<(), _>(why)
                    .into_iron_result(status::Created, status::BadRequest)
            }
        }
    }
}

pub struct PostController;

impl PostController {
    pub fn get_post(req: &mut Request) -> IronResult<Response> {
        use schema::posts::dsl::*;
        unimplemented!()
    }

    pub fn add_post(req: &mut Request) -> IronResult<Response> {
        let create_request: CreatePostRequest = jtry!(read_json_body(req));
        let user_id = jexpect!(req.extensions.get::<Router>().unwrap().find("user_id"));
        let token = jexpect!(req.extensions.get::<JwtToken>());
        if token.is_authenticated(user_id) {
            let conn = req.db_conn();
            let service = PostService::new(&*conn);
            service.insert_post(create_request)
                .into_iron_result(status::Created, status::InternalServerError)
        } else {
            JsonResponse::Error::<(), _>(Error::Other(String::from("Invalid credentials")))
                .into_iron_result(status::Created, status::Unauthorized)
        }
    }
}
