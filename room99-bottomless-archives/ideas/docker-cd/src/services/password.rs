#![allow(dead_code)] // @TODO: remove

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use eyre::Result;
use rand_core::OsRng;

pub fn hash_password(argon: Argon2<'_>, password: &[u8]) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    Ok(argon
        .hash_password(password, &salt)
        .map_err(|e| eyre::eyre!("Could not hash password because: {}", e))?
        .to_string())
}

pub fn verify_password(argon: Argon2<'_>, password: &[u8], hash: &str) -> Result<bool> {
    Ok(argon
        .verify_password(
            password,
            &PasswordHash::new(hash).map_err(|e| {
                eyre::eyre!(
                    "Could not convert the hash to a password hash because: {}",
                    e
                )
            })?,
        )
        .is_ok())
}
