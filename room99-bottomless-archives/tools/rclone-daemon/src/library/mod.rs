pub use ah_tools::{is_env, serde_is_valid_and_contain, set_var, var};

mod functions;
pub mod structs {
    pub use super::functions::{Mount, RcloneConfig, RcloneParser};
}

pub mod methods {
    pub use super::functions::config_management::{
        add_config, delete_config, dump_config, parse_config,
    };
    pub use super::functions::occ_commands::*;
    pub use super::functions::service_management::*;
}

pub mod reponses;
pub use reponses::{GenericResponse, Response, ResponseTypes};

mod requests;
pub mod queries {
    pub use super::requests::*;
}

pub mod endpoints;

#[macro_use]
pub mod defaults;
