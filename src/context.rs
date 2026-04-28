use std::{
    cell::Cell,
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
    /// mark whether the data is used mutably, if so, it should be saved before drop
    /// so we won't forget to save the data after mutating it
    pub mut_mark: Cell<bool>,
}

impl<T> Deref for WithContext<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for WithContext<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.mut_mark.set(true);
        &mut self.data
    }
}

impl<T> Drop for WithContext<'_, T> {
    fn drop(&mut self) {
        if self.mut_mark.get() {
            panic!("data is used mutably, but not saved");
        }
    }
}

impl Context {
    fn wrap<T>(&self, x: T) -> WithContext<'_, T> {
        WithContext {
            ctx: self,
            data: x,
            mut_mark: Cell::new(false),
        }
    }

    pub fn load_config(&self) -> anyhow::Result<WithContext<'_, Config>> {
        match &self.config_path {
            Some(x) => Config::load(x),
            None => Config::load_or_generate(Config::default_config_path()?),
        }
        .context("cannot load config")
        .map(|data| self.wrap(data))
    }
}
