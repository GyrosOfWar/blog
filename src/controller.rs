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

fn read_json_body<T>(req: &mut Request) -> Result<T> 
    where T: Deserialize
{
    let mut body = String::new();
    try!(req.body.read_to_string(&mut body));
    debug!("Request body: {}", body);
    ::serde_json::from_str(&body).map_err(From::from)
}

pub struct UserController;

impl UserController {
    pub fn make_jwt_token(req: &mut Request) -> IronResult<Response> {
        let conn = req.db_conn();
        let service = UserService::new(&*conn);
        match read_json_body::<UserCredentials>(req) {
            Ok(creds) => {
                let token_resp = service.make_token(&creds.name, &creds.password, super::SECRET);
                let json = itry!(::serde_json::to_string(&token_resp));
                Ok(Response::with((status::Ok, json)))
            }
            Err(why) => {
                Ok(Response::with((status::BadRequest, format!("Error: {}", why))))
            }
        }
    }

    
    pub fn create_user(req: &mut Request) -> IronResult<Response> {
        let conn = req.db_conn();
        let service = UserService::new(&*conn);
        match read_json_body::<CreateUserRequest>(req) {
            Ok(req) => {
                let resp = service.create_user(req);
                let json = itry!(::serde_json::to_string(&resp));
                Ok(Response::with(json))
            }
            Err(why) => {
                Ok(Response::with((status::BadRequest, format!("Error: {:?}", why))))
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
        let user_id = iexpect!(req.extensions.get::<Router>().unwrap().find("user_id"));
        let token = iexpect!(req.extensions.get::<JwtToken>());
        if token.is_authenticated(user_id) {
            let create_request: CreatePostRequest = itry!(read_json_body(req));
            let conn = req.db_conn();
            let service = PostService::new(&*conn);
            let resp = service.insert_post(create_request);
            resp.to_iron_result()
        } else {
            Ok(Response::with((status::Unauthorized, "Invalid credentials")))
        }

    }
}