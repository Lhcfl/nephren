use std::{
    env,
    fs::{self, File, OpenOptions},
    io::ErrorKind,
    path::{Path, PathBuf},
};

use anyhow::Context;
use log::{debug, info};
use serde::{Deserialize, Serialize};

use crate::{
    context::WithContext,
    models::{
        node::{Node, NodeId},
        subscription::Subscription,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub subscriptions: Vec<Subscription>,
    pub nodes: Vec<Node>,
    pub active_node: Option<NodeId>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            subscriptions: vec![],
            nodes: vec![],
            active_node: None,
        }
    }
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
        let path = path.as_ref();
        let file = File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn generate<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
        let path = path.as_ref();
        let config = Config::default();
        config.write_into(path)?;
        Ok(config)
    }

    pub fn write_into<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let path = path.as_ref();
        // recursively create parent dir
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    pub fn load_or_generate<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
        let path = path.as_ref();
        if fs::exists(&path)? {
            debug!("config file exists at {path:?}, loading it");
            Self::load(&path)
        } else {
            debug!("config file does not exist at {path:?}, generating it");
            Self::generate(&path)
        }
    }

    pub fn default_config_path() -> anyhow::Result<PathBuf> {
        let ret = env::home_dir()
            .context("failed to get your home dir. your system may not be supported.")?
            .join(".config/nephren/config.json");
        Ok(ret)
    }

    pub fn find_subscription(&self, id_or_name: &str) -> Option<usize> {
        self.subscriptions
            .iter()
            .position(|sub| sub.id.matches(id_or_name))
            .or_else(|| {
                self.subscriptions
                    .iter()
                    .position(|sub| sub.name == id_or_name)
            })
    }
}

impl WithContext<'_, Config> {
    pub fn save(&self) -> anyhow::Result<()> {
        self.mut_mark.set(false);

        match self.ctx.config_path.as_deref() {
            Some(x) => self.data.write_into(x),
            None => self.data.write_into(Config::default_config_path()?),
        }
    }
}
