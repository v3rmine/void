use super::User;
use crate::LOGGED;
use crate::{IV, KEY};
use ah_tools::reexports::encode;
use ah_tools::security::sym_dec;
use ah_tools::security::sym_enc_with_params;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::convert::From;

type Error = Box<dyn std::error::Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    // issuer
    email: String,
    // issued at
    iat: i64,
    // expiry
    exp: i64,
}

impl Claims {
    fn with_email(email: &str) -> Self {
        Claims {
            email: email.to_owned(),
            iat: Local::now().timestamp(),
            exp: (Local::now().timestamp() + 3_600_000),
        }
    }
}

impl From<Claims> for User {
    fn from(claims: Claims) -> Self {
        User {
            email: claims.clone().email,
            last_claim: claims,
        }
    }
}

pub fn create_token(email: String) -> Result<String, Error> {
    let mut lock_u: std::sync::MutexGuard<Vec<User>> =
        LOGGED.lock().map_err(|_| "Error checking the connection")?;
    let claims = Claims::with_email(email.as_str());
    let res = sym_enc_with_params(
        serde_json::to_string(&claims).map_err(|_| "Error decoding claim")?,
        &KEY.to_vec(),
        &IV.to_vec(),
    )?;
    lock_u.retain(|x| x.last_claim.iat < Local::now().timestamp());
    lock_u.push(User::from(claims));
    Ok(res)
}

pub fn mail_from_token(token: String) -> Result<String, Error> {
    let decode = sym_dec(encode(&KEY.to_vec()), encode(&IV.to_vec()), token)?;
    let claim = serde_json::from_str::<Claims>(String::from_utf8(decode)?.as_str())?;
    Ok(claim.email)
}

pub fn validate_token(token: &str) -> Result<bool, Error> {
    let token = sym_dec(
        encode(&KEY.to_vec()),
        encode(&IV.to_vec()),
        token.to_owned(),
    )?;
    let claims = serde_json::from_str::<Claims>(
        String::from_utf8(token)
            .map_err(|_| "Error decoding token")?
            .as_str(),
    )
    .map_err(|_| "Error decoding token")?;
    if claims.exp < Local::now().timestamp() {
        Ok(false)
    } else {
        Ok(true)
    }
}
