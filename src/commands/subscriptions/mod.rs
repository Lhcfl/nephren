use crate::commands::Exec;
use crate::context::Context;
use clap::{Args, Subcommand};

mod add;
mod list;
mod pull;
mod remove;

pub use add::Add;
pub use list::List;
pub use pull::Pull;
pub use remove::Remove;

#[derive(Debug, Args)]
pub struct Subscription {
    // #[command(subcommand)]
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// List all subscriptions
    List(List),
    /// Add a new subscription
    Add(Add),
    /// Remove a subscription
    #[command(visible_aliases(["rm", "r"]))]
    Remove(Remove),
    /// Pull updates for a subscription
    Pull(Pull),
}

impl Exec for Subscription {
    async fn exec(self, ctx: Context) -> anyhow::Result<()> {
        match self.cmd {
            Cmd::List(x) => x.exec(ctx).await,
            Cmd::Add(x) => x.exec(ctx).await,
            Cmd::Remove(x) => x.exec(ctx).await,
            Cmd::Pull(x) => x.exec(ctx).await,
        }
    }
}
