use crate::commands::Exec;
use clap::{Args, Subcommand};
pub mod add;
pub mod list;

#[derive(Debug, Args)]
pub struct Subscription {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    List(list::List),
    Add(add::Add),
}

impl Exec for Subscription {
    fn exec(self) -> anyhow::Result<()> {
        match self.cmd {
            Cmd::List(x) => x.exec(),
            Cmd::Add(x) => x.exec(),
        }
    }
}
