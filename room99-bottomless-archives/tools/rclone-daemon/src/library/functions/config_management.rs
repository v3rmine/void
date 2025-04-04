use super::occ_commands::execute_occ_scan;
use super::service_management::{create_service, mount_config};
use crate::defaults;
use crate::library::functions::service_management::unmount;
use pest::Parser;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{Read, Write};

/* === RCLONE CONFIG ===  */
#[derive(Debug, Clone)]
pub struct RcloneConfig {
    pub r#type: String,
    pub sa_id: String,
    pub client_id: String,
    pub client_secret: String,
    pub scope: String,
    pub service_account_file: String,
    pub trashed_only: bool,
    pub use_trash: bool,
    pub shared_with_me: bool,
    pub list_chunk: u32,
    pub chunk_size: u32,
    pub pacer_burst: u32,
    pub team_drive: String
}

impl std::convert::TryFrom<HashMap<String, String>> for RcloneConfig {
    type Error = Box<dyn std::error::Error>;

    fn try_from(hash: HashMap<String, String>) -> Result<Self, Self::Error> {
        Ok(RcloneConfig {
            r#type: hash.get("type").ok_or("ERROR")?.clone(),
            sa_id: hash.get("id").ok_or("ERROR")?.clone(),
            client_id: hash.get("client_id").ok_or("ERROR")?.clone(),
            client_secret: hash.get("client_secret").ok_or("ERROR")?.clone(),
            scope: hash.get("scope").ok_or("ERROR")?.clone(),
            service_account_file: hash.get("service_account_file").ok_or("ERROR")?.clone(),
            trashed_only: hash.get("trashed_only").ok_or("ERROR")?.clone().parse::<bool>().expect("Cannot parse dat bool"),
            use_trash: hash.get("use_trash").ok_or("ERROR")?.clone().parse::<bool>().expect("Cannot parse dat bool"),
            shared_with_me: hash.get("shared_with_me").ok_or("ERROR")?.clone().parse::<bool>().expect("Cannot parse dat bool"),
            list_chunk: hash.get("list_chunk").ok_or("ERROR")?.clone().parse::<u32>().expect("Cannot parse dat int"),
            chunk_size: hash.get("chunk_size").ok_or("ERROR")?.clone().parse::<u32>().expect("Cannot parse dat int"),
            pacer_burst: hash.get("pacer_burst").ok_or("ERROR")?.clone().parse::<u32>().expect("Cannot parse dat int"),
            team_drive: hash.get("team_drive").ok_or("ERROR")?.clone()
        })
    }
}

#[derive(Parser)]
#[grammar = "pest/rclone-conf.pest"]
pub struct RcloneParser;
/* === === ===  */

pub fn parse_config(path: String) -> Result<Vec<RcloneConfig>, Box<dyn std::error::Error>> {
    let mut unparsed_file = String::new();
    let mut f = File::open(path)?;
    f.read_to_string(&mut unparsed_file)?;
    let file = RcloneParser::parse(Rule::file, &unparsed_file)?
        .next()
        .unwrap();
    let mut result: Vec<RcloneConfig> = Vec::new();
    for line in file.into_inner() {
        match line.as_rule() {
            Rule::block => {
                let mut block = line.into_inner();
                let mut map: HashMap<String, String> = HashMap::new();
                map.insert("id".to_owned(), block.next().unwrap().as_str().to_owned());
                for kv in block {
                    let mut kv = kv.into_inner();
                    let key = kv.next().unwrap().as_str();
                    let value = kv.next().unwrap().as_str();
                    map.insert(key.to_string(), value.to_string());
                }
                result.push(RcloneConfig::try_from(map).expect("Cannot parse config file!"))
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    Ok(result)
}
pub fn add_config(conf: RcloneConfig, delete: bool) -> Result<(), Box<dyn std::error::Error>> {
    let confs = parse_config(defaults::CONFIG_PATH.to_owned()).map_err(|x| {
        println!("{:?}", x.to_string());
        ""
    })?;
    println!("{:?}", confs);
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(defaults::CONFIG_PATH)?;

    if confs.iter().find(|x| x.sa_id == conf.sa_id).is_none() {
        let mut txt = super::TEMPLATE_CONFIG.to_owned();
        txt = replace!(
            txt.clone(),
            sa_id = conf.clone().sa_id,
            type = conf.clone().r#type,
            client_id = conf.clone().client_id,
            client_secret = conf.clone().client_secret,
            scope = conf.clone().scope,
            service_account_file_path = conf.clone().service_account_file,
            shared_with_me = conf.clone().shared_with_me.to_string()
        );

        file.write_all(txt.as_bytes())?;
        if !delete {
            create_service(conf.clone().sa_id.as_str())?;
            let mut buf = format!(
                "#!/bin/sh\nsudo su www-data -s /bin/sh -c \"rclone mount {sa_id}: /var/nc-data/{sa_id}/files --drive-use-trash=false\"",
                sa_id = conf.clone().sa_id
            ).as_bytes().to_vec();
            File::open(format!(
                "{}/{}{}.sh",
                defaults::COMMANDS_DIR,
                defaults::PREFIX_SERVICE,
                conf.clone().sa_id
            ))
            .unwrap()
            .write_all(&mut buf)?;
        }
    }
    if !delete {
        mount_config(conf.clone().sa_id.as_str())?;
        execute_occ_scan(conf.sa_id.as_str())?;
    }

    Ok(())
}
pub fn delete_config(conf: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut confs = parse_config(defaults::CONFIG_PATH.to_owned())?;
    println!("{:?}", confs);
    let file = std::fs::OpenOptions::new()
        .write(true)
        .open(defaults::CONFIG_PATH)?;
    file.set_len(0)?;
    confs.retain(|config| config.sa_id != conf);
    println!("{:?}", confs);

    unmount(conf)?;
    std::fs::remove_file(format!(
        "{}/{}{}.service",
        defaults::SERVICE_DIR,
        defaults::PREFIX_SERVICE,
        conf
    ))?;
    std::fs::remove_file(format!(
        "{}/{}{}.sh",
        defaults::COMMANDS_DIR,
        defaults::PREFIX_SERVICE,
        conf
    ))?;

    for conf in confs {
        add_config(conf, true)?;
    }

    Ok(())
}
pub fn dump_config(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(std::fs::read_to_string(path)?)
}
