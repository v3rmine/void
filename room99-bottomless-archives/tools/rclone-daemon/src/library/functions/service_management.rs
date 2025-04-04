use super::execute_command_as;
use super::occ_commands::execute_occ_scan;
use crate::library::defaults;
use crate::library::functions::execute_command;
use std::io::{Read, Write};

pub fn mount_config(id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut result: Vec<String> = Vec::new();
    result.push(String::from_utf8(
        execute_command_as(
            "",
            format!("systemctl start {}{}", defaults::PREFIX_SERVICE, id).as_str(),
        )
        .output()?
        .stdout,
    )?);
    result.push(match execute_occ_scan(id)?.stdout {
        Some(mut x) => {
            let mut res = String::new();
            x.read_to_string(&mut res)
                .map_err(|_| "Error while executing the scan")?;
            res
        }
        None => "".to_owned(),
    });
    Ok(result.join("\n"))
}
pub fn enable(id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let result = String::from_utf8(
        execute_command_as(
            "",
            format!("systemctl enable {}{}", defaults::PREFIX_SERVICE, id).as_str(),
        )
        .output()?
        .stdout,
    )?;
    Ok(result)
}
pub fn disable(id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let result = String::from_utf8(
        execute_command_as(
            "",
            format!("systemctl disable {}{}", defaults::PREFIX_SERVICE, id).as_str(),
        )
        .output()?
        .stdout,
    )?;
    Ok(result)
}
pub fn unmount(mount: &str) -> Result<String, Box<dyn std::error::Error>> {
    disable(mount)?;
    Ok(String::from_utf8(
        execute_command_as("", format!("systemctl stop {}", mount).as_str())
            .output()?
            .stdout,
    )?)
}
pub fn unmount_all() -> Result<String, Box<dyn std::error::Error>> {
    let mut result: Vec<String> = Vec::new();
    for service in service_list()? {
        result.push(unmount(service.as_str())?);
        result.push(fusermount(service.as_str()))
    }
    Ok(result.join("\n"))
}
pub fn fusermount(path: &str) -> String {
    |path: &str| -> Result<String, Box<dyn std::error::Error>> {
        Ok(String::from_utf8(
            match execute_command_as("", format!("fusermount -u {}", path).as_str()).output() {
                Ok(x) => x.stdout,
                _ => b"".to_vec(),
            },
        )?)
    }(path)
    .unwrap_or_else(|_| "".to_owned())
}
pub fn kill_proc(pid: u64) -> Result<String, Box<dyn std::error::Error>> {
    Ok(String::from_utf8(
        execute_command_as("", format!("kill -SIGTERM {}", pid).as_str())
            .output()?
            .stdout,
    )?)
}

pub fn create_service(id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = format!(
        "{}/{}{}.service",
        defaults::SERVICE_DIR,
        defaults::PREFIX_SERVICE,
        id
    );
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path.as_str())?;

    let mut txt = super::TEMPLATE_SERVICE.to_owned();
    txt = replace!(
        txt.clone(),
        sa_id = id.to_owned(),
        file = format!(
            "{}/{}{}.sh",
            defaults::COMMANDS_DIR,
            defaults::PREFIX_SERVICE,
            id
        )
    );

    file.write_all(txt.as_bytes())?;
    execute_command(
        format!(
            "ln -s {p}/{prefix}{name}.service /etc/systemd/system/",
            p = defaults::SERVICE_DIR,
            prefix = defaults::PREFIX_SERVICE,
            name = id
        )
        .as_str(),
    );

    let mut result: Vec<String> = Vec::new();
    result.push(mount_config(id)?);
    result.push(enable(id)?);

    Ok(result.join("\n"))
}
fn service_list() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let list = execute_command_as(
        "",
        format!(
            "systemctl list-units --type=service --state=running | grep -o ^{}.*.service",
            defaults::PREFIX_SERVICE
        )
        .as_str(),
    )
    .output()?
    .stdout;
    let list = String::from_utf8(list)?;
    let list = list.split('\n');
    let list: Vec<&str> = list.collect();
    let mut result: Vec<String> = Vec::new();
    for proc in list.iter() {
        let tmp = proc.split(".service");
        let tmp: &str = tmp.collect::<Vec<&str>>()[0];
        let tmp = tmp.split(defaults::PREFIX_SERVICE);
        let tmp: &str = tmp.collect::<Vec<&str>>()[0];
        result.push(tmp.to_string());
    }
    Ok(result)
}
