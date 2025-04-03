use std::fs::{read_to_string, write};
use std::ops::Deref;
use std::path::PathBuf;

use dirs::home_dir;

use crate::errors::{BoilrError, StandardResult};
use crate::utils::check_if_install_dir_exist;
use crate::utils::types::{Config, Template};
use crate::{CONFIG_FILE_NAME, INSTALL_DIR};

#[derive(Debug, Clone)]
pub struct ConfigIO {
    pub config: Config,
    pub path: Option<PathBuf>,
    pub dir: Option<PathBuf>,
}

impl ConfigIO {
    pub fn new() -> StandardResult<Self> {
        let mut config = Self::default();
        let dir_path = Self::get_dir_path()?;
        config.path = Some(dir_path.join(CONFIG_FILE_NAME));
        config.dir = Some(dir_path);
        config.update_config()?;
        Ok(config)
    }

    pub fn get_path() -> StandardResult<PathBuf> {
        check_if_install_dir_exist()?;
        let install_directory_path = home_dir()
            .ok_or(BoilrError::HomeDirNotFoundError)?
            .join(INSTALL_DIR);

        Ok(install_directory_path.join(CONFIG_FILE_NAME))
    }

    pub fn get_dir_path() -> StandardResult<PathBuf> {
        check_if_install_dir_exist()?;
        let install_directory_path = home_dir()
            .ok_or(BoilrError::HomeDirNotFoundError)?
            .join(INSTALL_DIR);

        Ok(install_directory_path)
    }

    pub fn parse_config(path: &PathBuf) -> StandardResult<Config> {
        let config = toml::from_str::<Config>(&read_to_string(path).map_err(|source| {
            BoilrError::ReadError {
                source,
                path: path.clone(),
            }
        })?)
        .map_err(|source| BoilrError::TomlDeserializeError {
            source,
            path: path.clone(),
        })?;
        Ok(config)
    }

    pub fn write_config(&self) -> StandardResult<Self> {
        let config_path = match &self.path {
            Some(path) => path.to_owned(),
            None => Self::get_path()?,
        };

        write(
            &config_path,
            toml::to_string(&self.config)
                .map_err(|source| BoilrError::TomlSerializeError {
                    source,
                    path: config_path.clone(),
                })?
                .as_bytes(),
        )
        .map_err(|source| BoilrError::WriteError {
            source,
            path: config_path,
        })?;

        Ok(self.clone())
    }

    pub fn update_config(&mut self) -> StandardResult<Self> {
        let config_path = match &self.path {
            Some(path) => path.to_owned(),
            None => Self::get_path()?,
        };

        self.config = Self::parse_config(&config_path)?;

        Ok(self.clone())
    }

    pub fn push_template(
        &mut self,
        template_name: &str,
        template_path: &PathBuf,
    ) -> StandardResult<Self> {
        self.config.templates.push(Template {
            name: template_name.to_owned(),
            path: template_path
                .to_str()
                .ok_or(BoilrError::StrError)?
                .to_owned(),
        });

        Ok(self.clone())
    }

    pub fn retain_templates<F>(&mut self, f: F) -> Self
    where
        F: FnMut(&Template) -> bool,
    {
        self.config.templates.retain(f);
        self.clone()
    }

    pub fn find_index<F>(&mut self, mut f: F) -> Option<usize>
    where
        F: FnMut(&Template) -> bool,
    {
        let mut result = None;

        for (idx, template) in self.iter().enumerate() {
            if f(template) {
                result = Some(idx);
                break;
            }
        }

        result
    }
}

impl Default for ConfigIO {
    fn default() -> Self {
        ConfigIO {
            config: Config {
                templates: Vec::new(),
            },
            path: None,
            dir: None,
        }
    }
}

impl From<&PathBuf> for ConfigIO {
    fn from(path: &PathBuf) -> Self {
        let mut config = Self::default();
        config.path = Some(path.join(CONFIG_FILE_NAME));
        config.dir = Some(path.to_owned());
        config
    }
}

impl Deref for ConfigIO {
    type Target = Vec<Template>;

    fn deref(&self) -> &Self::Target {
        &self.config.templates
    }
}
