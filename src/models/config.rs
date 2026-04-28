use std::{
    env,
    fs::{self, File},
    io::ErrorKind,
};

use log::info;
use serde::{Deserialize, Serialize};

use crate::models::{
    node::{Node, NodeId},
    subscription::Subscription,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    subscriptions: Vec<Subscription>,
    nodes: Vec<Node>,
    active_node: Option<NodeId>,
}

pub enum ConfigLoadError {
    DeserializeError(String),
    NotFound,
    FileSystemError(String),
}

impl Config {
    pub fn load() -> Result<Config, ConfigLoadError> {
        let Some(home) = env::home_dir() else {
            panic!("failed to get your home dir. your system may not be supported.");
        };

        let path = home.join("/.config/nephren/config.json");

        let file = File::open(path).map_err(|err| {
            use ConfigLoadError::*;
            match err.kind() {
                ErrorKind::NotFound => NotFound,
                _ => FileSystemError(err.to_string()),
            }
        })?;

        serde_json::from_reader(file)
            .map_err(|err| ConfigLoadError::DeserializeError(err.to_string()))
    }

    pub fn load_or_generate() -> Config {
        use ConfigLoadError::*;

        Config::load().unwrap_or_else(|e| match e {
            DeserializeError(msg) => {
                eprintln!("======= UNRECOVERABLE ERROR OCURRERS! ========");
                eprintln!("{msg}");
                panic!("cannot deserialize config.")
            }
            FileSystemError(msg) => {
                eprintln!("======= UNRECOVERABLE ERROR OCURRERS! ========");
                eprintln!("{msg}");
                panic!("unhandled filesystem error.")
            }
            NotFound => {
                info!("no config file detected. creating one...");

                let ret = Config {
                    subscriptions: vec![],
                    nodes: vec![],
                    active_node: None,
                };

                // serde_json::to_writer();

                ret
            }
        })
    }
}
