use std::str::FromStr;

use iron::prelude::*;
use router::Router;
use urlencoded::UrlEncodedQuery;

pub trait UrlParamExt {
    fn url_param<T: FromStr>(&mut self, name: &str) -> Option<T>;
}

pub trait PathParamExt {
    fn path_param<T: FromStr>(&mut self, name: &str) -> Option<T>;
}

impl<'a, 'b> UrlParamExt for Request<'a, 'b> {
    fn url_param<T: FromStr>(&mut self, name: &str) -> Option<T> {
        let params = self.get_ref::<UrlEncodedQuery>().ok();
        params.and_then(|map| map.get(name).and_then(|v| v.get(0).and_then(|p| p.parse::<T>().ok())))
    }
}

impl<'a, 'b> PathParamExt for Request<'a, 'b> {
    fn path_param<T: FromStr>(&mut self, name: &str) -> Option<T> {
        self.extensions.get::<Router>().and_then(|r| r.find(name).and_then(|p| p.parse::<T>().ok()))
    }
}
