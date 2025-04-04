use std::process::Command;

pub fn execute(app: &str, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    Ok(String::from_utf8(
        Command::new(format!(
            "{}{}{}",
            std::env::current_exe()?
                .parent()
                .ok_or("Folder not found")?
                .to_str()
                .ok_or("Folder not found")?,
            super::defaults::EXECUTABLE_PATH,
            app
        ))
        .args(args)
        .output()?
        .stdout,
    )?)
}

pub fn execute_with_envs(
    app: &str,
    args: &[&str],
    envs: Vec<(&str, &str)>,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok(String::from_utf8(
        Command::new(format!(
            "{}{}{}",
            std::env::current_exe()?
                .parent()
                .ok_or("Folder not found")?
                .to_str()
                .ok_or("Folder not found")?,
            super::defaults::EXECUTABLE_PATH,
            app
        ))
        .args(args)
        .envs(envs)
        .output()?
        .stdout,
    )?)
}
