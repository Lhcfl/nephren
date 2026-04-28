use crate::commands::Exec;
use crate::context::Context;
use clap::{Args, Subcommand};

pub mod list;

#[derive(Debug, Args)]
pub struct Nodes {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// List all nodes
    List(list::List),
}

impl Exec for Nodes {
    async fn exec(self, ctx: Context) -> anyhow::Result<()> {
        match self.cmd {
            Cmd::List(x) => x.exec(ctx).await,
        }
    }
}
