use crate::commands::Exec;
use clap::{Args, Subcommand};

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
    fn exec(self) -> anyhow::Result<()> {
        match self.cmd {
            Cmd::List(x) => x.exec(),
            Cmd::Add(x) => x.exec(),
            Cmd::Pull(x) => x.exec(),
        }
    }
}
