use bcrypt::{hash, DEFAULT_COST};
use bcrypt::verify;
use std::env;

pub fn hash_password(plain: &str) -> Result<String, Box<dyn std::error::Error>> {
    // get the hashing cost from the env variable or use default
    let hashing_cost: u32 = match env::var("HASH_ROUNDS") {
        Ok(cost) => cost.parse().unwrap_or(DEFAULT_COST),
        _ => DEFAULT_COST,
    };
    Ok(hash(plain, hashing_cost).map_err(|_| "Hashing error")?)
}


pub fn compare(pass1: &str, pass2: &str) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(verify(pass1, pass2)?)
}