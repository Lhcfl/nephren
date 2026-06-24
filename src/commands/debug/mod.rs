use crate::commands::Exec;
use crate::context::Context;
use clap::{Args, Subcommand};

mod parse;

pub use parse::Parse;

#[derive(Debug, Args)]
pub struct Debug {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// Parse a share link
    Parse(Parse),
}

impl Exec for Debug {
    async fn exec(self, ctx: Context) -> anyhow::Result<()> {
        match self.cmd {
            Cmd::Parse(p) => p.exec(ctx).await,
        }
    }
}
