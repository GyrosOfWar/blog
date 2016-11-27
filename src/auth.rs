use jwt::{Claims, Header, Registered, Token};
use crypto::sha2::Sha256;
use iron::BeforeMiddleware;
use iron::headers::{Authorization, Bearer};
use iron::prelude::*;
use iron::typemap::Key;

const DEFAULT_EXPIRATION_TIME: u64 = 24 * 60 * 60;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCredentials {
    pub name: String,
    pub password: String,
}

pub struct TokenMaker {
    // TODO I can probably avoid an allocation here since 
    // the middleware owns the secret.
    server_secret: Vec<u8>,
    issuer: Option<String>,
    expiration_time: u64,
}

impl TokenMaker {
    pub fn new(server_secret: &[u8]) -> TokenMaker {
        TokenMaker {
            server_secret: server_secret.to_owned(),
            issuer: None,
            expiration_time: DEFAULT_EXPIRATION_TIME,
        }
    }

    pub fn make_token(&self, user_id: &str) -> Option<String> {
        let header: Header = Default::default();
        let now = current_numeric_date();
        let claims = Claims {
            reg: Registered {
                iss: self.issuer.clone(),
                sub: Some(user_id.to_owned()),
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

pub struct JwtToken(pub Option<Token<Header, Claims>>);

impl JwtToken {
    pub fn is_authenticated(&self, user_id: i32) -> bool {
        match *self {
            JwtToken(Some(ref token)) => {
                match token.claims.reg.sub {
                    Some(ref sub) => {
                        match sub.parse::<i32>().ok() {
                            Some(parsed) => parsed == user_id,
                            None => false
                        }
                    },
                    None => false,
                }
            }
            JwtToken(None) => false,
        }
    }
}

impl Key for JwtToken {
    type Value = JwtToken;
}

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
}

fn current_numeric_date() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).ok().unwrap().as_secs()
}

impl BeforeMiddleware for JwtMiddleware {
    fn before(&self, request: &mut Request) -> IronResult<()> {
        debug!("JwtMiddleware::before({:?})", request);
        match request.headers.get::<Authorization<Bearer>>() {
            Some(h) => {
                let jwt_str = &h.token;
                debug!("Received JWT {}", jwt_str);
                match Token::<Header, Claims>::parse(jwt_str) {
                    Ok(token) => {
                        debug!("Parsed token: {:?}", token);
                        if token.verify(&self.server_secret, Sha256::new()) {
                            let now = current_numeric_date();
                            if let Some(exp) = token.claims.reg.exp {
                                if now < exp {
                                    request.extensions.insert::<JwtToken>(JwtToken(Some(token)));
                                    return Ok(());
                                }
                            }
                        }
                    }
                    Err(why) => {
                        debug!("Bad JWT: {:?}", why);
                    }
                }
            }
            None => {
                debug!("No authorization token found.");
            }
        }
        request.extensions.insert::<JwtToken>(JwtToken(None));
        Ok(())
    }

    fn catch(&self, _: &mut Request, _: IronError) -> IronResult<()> {
        Ok(())
    }
}
