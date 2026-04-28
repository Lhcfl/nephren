use crate::commands::Exec;
use crate::context::Context;
use clap::{Args, Subcommand};
use enum_dispatch::enum_dispatch;

pub mod add;
pub mod list;
pub mod pull;

#[derive(Debug, Args)]
pub struct Subscription {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    List(list::List),
    Add(add::Add),
    Pull(pull::Pull),
}

impl Exec for Subscription {
    fn exec(self, ctx: Context) -> anyhow::Result<()> {
        match self.cmd {
            Cmd::List(x) => x.exec(ctx),
            Cmd::Add(x) => x.exec(ctx),
            Cmd::Pull(x) => x.exec(ctx),
        }
    }
}
