use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::BTreeMap;
use worker::Request;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Authorization {
    pub success: bool,
    pub username: String,
}

pub fn create_user(username: &str, secret: String) -> String {
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("username", username);

    let token_str = claims.sign_with_key(&key).unwrap();

    token_str
}

pub fn authorize(req: &Request, secret: String) -> Authorization {
    let token_str = match req.headers().get("authorization") {
        Ok(token) => match token {
            Some(t) => t,
            None => {
                return Authorization {
                    success: false,
                    username: "".to_owned(),
                }
            }
        },
        Err(_) => {
            return Authorization {
                success: false,
                username: "".to_owned(),
            }
        }
    };

    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
    let claims: BTreeMap<String, String> = match token_str.verify_with_key(&key) {
        Ok(body) => body,
        Err(_) => {
            return Authorization {
                success: false,
                username: "".to_owned(),
            }
        }
    };

    let username = &claims["username"];

    Authorization {
        success: true,
        username: username.to_string(),
    }
}
