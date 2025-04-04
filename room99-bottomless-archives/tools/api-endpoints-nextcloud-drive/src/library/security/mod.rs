mod jwt;
use crate::{LOGGED, USERS};
use ah_tools::passwords::compare;
pub use jwt::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub last_claim: jwt::Claims,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisteredUser {
    pub email: String,
    pub password: String,
}

type Error = Box<dyn std::error::Error>;

pub fn get_token(headers: &actix_web::http::HeaderMap) -> Result<String, Error> {
    if headers.contains_key("Authorization") {
        let x = headers
            .get("Authorization")
            .ok_or("Error while getting Authorization")?
            .to_str()
            .map_err(|_| "Error while getting Authorization")?;
        let x = x.split(' ').collect::<Vec<&str>>()[1];
        Ok(x.to_owned())
    } else {
        Err("Error cannot find 'Authorization' header")?
    }
}

pub fn connect(mail: &str, password: &str) -> Result<bool, Error> {
    let lock_u: std::sync::MutexGuard<Vec<User>> =
        LOGGED.lock().map_err(|_| "Error checking the connection")?;
    let lock_r: std::sync::MutexGuard<Vec<RegisteredUser>> =
        USERS.lock().map_err(|_| "Error checking the connection")?;
    if match lock_r.iter().find(|x| x.email == mail) {
        Some(_) => lock_u.iter().find(|x| x.email == mail).is_none(),
        None => false,
    } {
        if compare(
            password,
            lock_r
                .iter()
                .find(|x| x.email == mail)
                .unwrap()
                .password
                .as_str(),
        )? {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn is_connected(mail: &str) -> Result<bool, Error> {
    let lock_u: std::sync::MutexGuard<Vec<User>> =
        LOGGED.lock().map_err(|_| "Error checking the connection")?;
    let lock_r: std::sync::MutexGuard<Vec<RegisteredUser>> =
        USERS.lock().map_err(|_| "Error checking the connection")?;
    match lock_r.iter().find(|x| x.email == mail) {
        Some(_) => Ok(lock_u.iter().any(|x| x.email == mail)),
        None => Err("Error user not found")?,
    }
}

pub fn disconnect(mail: &str) -> Result<bool, Error> {
    let mut lock: std::sync::MutexGuard<Vec<User>> =
        LOGGED.lock().map_err(|_| "Error checking the connection")?;
    let ini_len = lock.len();
    lock.retain(|x| x.email != mail);
    if ini_len > lock.len() {
        Ok(true)
    } else {
        Ok(false)
    }
}
