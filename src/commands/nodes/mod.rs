use crate::commands::Exec;
use crate::context::Context;
use clap::{Args, Subcommand};

pub mod activate;
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
    /// Activate a node
    #[command(visible_aliases(["active", "a"]))]
    Activate(activate::Activate),
}

impl Exec for Nodes {
    async fn exec(self, ctx: Context) -> anyhow::Result<()> {
        match self.cmd {
            Cmd::List(x) => x.exec(ctx).await,
            Cmd::Activate(x) => x.exec(ctx).await,
        }
    }
}
