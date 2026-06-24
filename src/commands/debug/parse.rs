use crate::commands::Exec;
use crate::context::Context;
use crate::share::parse;
use clap::Args;
use log::debug;

#[derive(Debug, Args)]
pub struct Parse {
    text: String,
}

impl Exec for Parse {
    async fn exec(self, ctx: Context) -> anyhow::Result<()> {
        parse(&self.text)
    }
}
