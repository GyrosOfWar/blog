use std::sync::Arc;

use pencil::{PencilResult, Request, Module, jsonify};
use pencil::method::Get;

use super::APP;
use dao::{Dao, PostDao};
use util::JsonResponse;
use errors::*;

pub struct PostApi;

impl PostApi {
    fn get_post(request: &mut Request) -> PencilResult {
        let conn = Arc::new(try!(APP.conn_pool.get().map_err(|e| to_pencil_error(e))));
        let dao = PostDao::new(conn);
        let id = try!(request.view_args
            .get("id")
            .unwrap()
            .parse::<i32>()
            .map_err(|e| to_pencil_error(e)));
        let post = dao.get_one(&id);
        jsonify(&JsonResponse::from_result(post))
    }

    pub fn get_module() -> Module {
        let mut module = Module::new("posts", "/api/post");
        module.route("<id:int>", &[Get], "get_post", PostApi::get_post);
        module
    }
}
