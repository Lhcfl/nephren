use crate::commands::Exec;
use crate::context::Context;
use anyhow::bail;
use clap::{Args, Subcommand};
use log::error;
use tabled::grid::config;

#[derive(Debug, Args)]
pub struct Pull {
    id_or_name: String,
}

impl Exec for Pull {
    fn exec(self, ctx: Context) -> anyhow::Result<()> {
        let config = ctx.load_config()?;
        let matched_index = config.find_subscription(&self.id_or_name);
        if let Some(index) = matched_index {
            let matched = &config.subscriptions[index];
            println!("pulling subscription {} ({})", matched.id, matched.name);
            todo!()
        } else {
            bail!("subscription not found: {}", self.id_or_name);
        }
    }
}
