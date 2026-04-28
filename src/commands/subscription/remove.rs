use crate::context::Context;
use crate::{commands::Exec, models::subscription::Subscription};
use anyhow::bail;
use clap::{Args, Subcommand};
use log::error;

#[derive(Debug, Args)]
pub struct Remove {
    id_or_name: String,
}

impl Exec for Remove {
    fn exec(self, ctx: Context) -> anyhow::Result<()> {
        let mut config = ctx.load_config()?;

        let matched_index = config
            .subscriptions
            .iter()
            .position(|sub| sub.id.matches(&self.id_or_name))
            .or_else(|| {
                config
                    .subscriptions
                    .iter()
                    .position(|sub| sub.name == self.id_or_name)
            });

        if let Some(index) = matched_index {
            let matched = config.subscriptions.remove(index);
            config.save()?;
            println!("successfully removed subscription: {matched:#?}");
        } else {
            bail!("subscription not found: {}", self.id_or_name);
        }

        Ok(())
    }
}
