use jwt::{Claims, Header, Registered, Token};
use crypto::sha2::Sha256;
use iron::BeforeMiddleware;
use iron::prelude::*;

const DEFAULT_EXPIRATION_TIME: u64 = 24 * 60 * 60;

#[derive(Clone)]
pub struct JwtMiddleware {
    server_secret: Vec<u8>,
    issuer: Option<String>,
    expiration_time: u64,
}

impl JwtMiddleware {
    pub fn new(server_secret: &[u8]) -> JwtMiddleware {
        JwtMiddleware {
            server_secret: server_secret.to_owned(),
            issuer: None,
            expiration_time: DEFAULT_EXPIRATION_TIME,
        }
    }

    pub fn issuer(&mut self, issuer: &str) -> &mut JwtMiddleware {
        self.issuer = Some(issuer.to_owned());
        self
    }

    pub fn expiration_time(&mut self, expiration_time: u64) -> &mut JwtMiddleware {
        self.expiration_time = expiration_time;
        self
    }

    fn make_token(&self, user: &str)
                  -> Option<String> {
        let header: Header = Default::default();
        let now = current_numeric_date();
        let claims = Claims {
            reg: Registered {
                iss: self.issuer.clone(),
                sub: Some(user.to_owned()),
                exp: Some(now + self.expiration_time),
                nbf: Some(now),
                ..Default::default()
            },
            ..Default::default()
        };
        let token = Token::new(header, claims);
        token.signed(self.server_secret.as_ref(), Sha256::new()).ok()
    }
}

fn current_numeric_date() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).ok().unwrap().as_secs()
}

impl BeforeMiddleware for JwtMiddleware {
    fn before(&self, request: &mut Request) -> IronResult<()> {
        unimplemented!()
    }

    fn catch(&self, request: &mut Request, err: IronError) -> IronResult<()> {
        unimplemented!()
    }
}