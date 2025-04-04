pub use std::env::{set_var, var};

pub fn is_env(
    env: &str,
    exist: &dyn Fn(&str),
    notexist: &dyn Fn(&str),
) -> Result<(), Box<dyn std::error::Error>> {
    if var(env).is_err() || var(env)? == "" {
        notexist(env);
    } else {
        exist(env);
    }
    Ok(())
}