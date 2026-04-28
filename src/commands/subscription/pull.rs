use std::ops::DerefMut;

use crate::context::Context;
use crate::models::config::Config;
use crate::{commands::Exec, inter::subscription::parse_base64_input};
use anyhow::{Ok, bail};
use clap::Args;
use log::warn;

#[derive(Debug, Args)]
pub struct Pull {
    id_or_name: String,
}

impl Exec for Pull {
    async fn exec(self, ctx: Context) -> anyhow::Result<()> {
        let mut config = ctx.load_config()?;
        let matched_index = config.find_subscription(&self.id_or_name);
        if let Some(index) = matched_index {
            let Config {
                subscriptions,
                nodes,
                ..
            } = config.deref_mut();

            let matched = &subscriptions[index];

            let Some(url) = &matched.url else {
                bail!("subscription url is empty");
            };

            println!("pulling subscription {} ({})", matched.id, matched.name);

            let resp = reqwest::get(url).await?.text().await?;
            let (mut new_nodes, failed) = parse_base64_input(&resp)?;

            if new_nodes.is_empty() {
                bail!("no valid nodes found in subscription. {failed} nodes failed to parse.");
            }

            // remove old nodes that belong to this subscription
            nodes.retain(|x| x.belongs_to != Some(matched.id));

            let mut last_id = nodes.iter().map(|x| x.id).max().unwrap_or_default();

            for node in &mut new_nodes {
                last_id = last_id.next();
                node.id = last_id;
                node.belongs_to = Some(matched.id);
            }

            // add new nodes to config
            nodes.append(&mut new_nodes);

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
