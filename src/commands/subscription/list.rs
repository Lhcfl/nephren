use crate::commands::Exec; use crate::context::Context;
use anyhow::bail;
use clap::{Args, Subcommand};
use log::error;

#[derive(Debug, Args)]
pub struct List {}

impl Exec for List {
    fn exec(self, ctx: Context) -> anyhow::Result<()> {
        bail!("not listing")
    }
}
