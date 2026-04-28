use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use anyhow::Context as _;

use crate::models::config::Config;

pub struct Context {
    pub config_path: Option<PathBuf>,
}

pub struct WithContext<'a, T> {
    pub ctx: &'a Context,
    pub data: T,
}

impl Deref for WithContext<'_, Config> {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for WithContext<'_, Config> {
    fn deref_mut(&mut self) -> &mut Config {
        &mut self.data
    }
}

impl Context {
    pub fn load_config(&self) -> anyhow::Result<WithContext<'_, Config>> {
        match &self.config_path {
            Some(x) => Config::load(x),
            None => Config::load_or_generate(Config::default_config_path()?),
        }
        .context("cannot load config")
        .map(|data| WithContext { ctx: self, data })
    }
}

impl WithContext<'_, Config> {
    pub fn save(&self) -> anyhow::Result<()> {
        match self.ctx.config_path.as_deref() {
            Some(x) => self.data.write_into(x),
            None => self.data.write_into(Config::default_config_path()?),
        }
    }
}
