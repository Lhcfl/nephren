use crate::commands::Exec;
use anyhow::bail;
use clap::{Args, Subcommand};
use log::error;

#[derive(Debug, Args)]
pub struct Pull {
    id_or_name: String,
}

impl Exec for Pull {
    fn exec(self) -> anyhow::Result<()> {
        bail!("not implemented")
    }
}
