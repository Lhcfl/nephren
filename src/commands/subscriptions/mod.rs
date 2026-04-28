use crate::commands::Exec;
use crate::context::Context;
use clap::{Args, Subcommand};

pub mod add;
pub mod list;
pub mod pull;
pub mod remove;

#[derive(Debug, Args)]
pub struct Subscription {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// List all subscriptions
    List(list::List),
    /// Add a new subscription
    Add(add::Add),
    /// Remove a subscription
    #[command(visible_aliases(["rm", "r"]))]
    Remove(remove::Remove),
    /// Pull updates for a subscription
    Pull(pull::Pull),
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
