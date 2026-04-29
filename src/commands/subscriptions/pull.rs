use std::ops::DerefMut;

use crate::context::Context;
use crate::models::config::Config;
use crate::{commands::Exec, inter::subscription::parse_base64_input};
use anyhow::{Ok, bail};
use clap::Args;
use log::{info, warn};

#[derive(Debug, Args)]
pub struct Pull {
    id_or_name: String,
}

impl Exec for Pull {
    async fn exec(self, ctx: Context) -> anyhow::Result<()> {
        let mut config = ctx.load_config()?;
        let matched_index = config.find_subscription(&self.id_or_name);
        if let Some(index) = matched_index {
            let matched = &config.subscriptions[index];
            let (new_nodes, failed) = matched.pull().await?;
            let matched_id = matched.id;

            config.replace_subscription_nodes(matched_id, new_nodes);
            config.save()?;

            if failed > 0 {
                warn!(
                    "{} nodes failed to parse, but the rest have been added",
                    failed
                );
            }

            Ok(())
        } else {
            bail!("subscription not found: {}", self.id_or_name);
        }
    }
}
