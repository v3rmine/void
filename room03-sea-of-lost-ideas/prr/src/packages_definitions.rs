use std::io;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PackageManager {
    OsxBrew,
    LinuxPacman,
    LinuxYay,
    LinuxApt,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackageWithManager {
    pub manager: PackageManager,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackageRaw {
    pub name: String,
    pub cmd: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum PackageDefinition {
    Raw(PackageRaw),
    WithManager(PackageWithManager),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PackagesDefinitions(Vec<PackageDefinition>);

impl PackagesDefinitions {
    pub fn get_from_stdio() -> Self {
        let stdin = io::stdin();
        let mut stdin_data = String::new();

        let mut line = String::new();
        while let Ok(nb_bytes) = stdin.read_line(&mut line) {
            if nb_bytes == 0 {
                break;
            };
            stdin_data.push_str(&line);
            line.clear();
        }

        match serde_json::from_str::<Self>(&stdin_data) {
            Ok(packages_defs) => packages_defs,
            Err(_) => PackagesDefinitions::default(),
        }
    }

    pub fn to_json(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(_) => String::from("[]"),
        }
    }
}
