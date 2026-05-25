use crate::commands::Exec;
use crate::context::Context;
use clap::{Args, Subcommand};

mod activate;
mod list;

pub use activate::Activate;
pub use list::List;

#[derive(Debug, Args)]
pub struct Nodes {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// List all nodes
    List(List),
    /// Activate a node
    #[command(visible_aliases(["active", "a"]))]
    Activate(Activate),
}

impl Exec for Nodes {
    async fn exec(self, ctx: Context) -> anyhow::Result<()> {
        match self.cmd {
            Cmd::List(x) => x.exec(ctx).await,
            Cmd::Activate(x) => x.exec(ctx).await,
        }
    }
}
