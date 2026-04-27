use crate::commands::Exec;
use anyhow::bail;
use clap::{Args, Subcommand};
use log::error;

#[derive(Debug, Args)]
pub struct List {}

impl Exec for List {
    fn exec(self) -> anyhow::Result<()> {
        bail!("not listing")
    }
}
