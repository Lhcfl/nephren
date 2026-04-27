use crate::commands::Exec;
use anyhow::bail;
use clap::{Args, Subcommand};
use log::error;

#[derive(Debug, Args)]
pub struct Add {
    url: String,
}

impl Exec for Add {
    fn exec(self) -> anyhow::Result<()> {
        bail!("not implemented")
    }
}
