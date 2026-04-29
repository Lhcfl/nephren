use std::{
    env,
    fs::{self, File, OpenOptions},
    path::{Path, PathBuf},
};

use anyhow::Context;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::{
    context::WithContext,
    models::{
        node::{Node, NodeId},
        subscription::{Subscription, SubscriptionId},
    },
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub subscriptions: Vec<Subscription>,
    pub nodes: Vec<Node>,
    pub active_node: Option<NodeId>,
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
        if fs::exists(path)? {
            debug!("config file exists at {path:?}, loading it");
            Self::load(path)
        } else {
            debug!("config file does not exist at {path:?}, generating it");
            Self::generate(path)
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

    pub fn find_node(&self, id_or_name: &str) -> Option<usize> {
        self.nodes
            .iter()
            .position(|node| node.id.matches(id_or_name))
            .or_else(|| self.nodes.iter().position(|node| node.name == id_or_name))
    }

    pub fn replace_subscription_nodes(
        &mut self,
        subscription_id: SubscriptionId,
        mut new_nodes: Vec<Node>,
    ) {
        self.nodes.retain(|x| x.belongs_to != Some(subscription_id));

        let mut last_id = self.nodes.iter().map(|x| x.id).max().unwrap_or_default();

        for node in &mut new_nodes {
            last_id = last_id.next();
            node.id = last_id;
            node.belongs_to = Some(subscription_id);
        }

        self.nodes.extend(new_nodes);
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
