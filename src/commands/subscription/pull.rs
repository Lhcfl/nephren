use crate::context::Context;
use crate::{commands::Exec, lib::subscription::parse_base64_input};
use anyhow::{Ok, bail};
use clap::Args;

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

            let Some(url) = &matched.url else {
                bail!("subscription url is empty");
            };

            println!("pulling subscription {} ({})", matched.id, matched.name);

            let resp = reqwest::get(url).await?.text().await?;

            println!("response: {}", resp);

            let mut last_id = config.nodes.iter().map(|x| x.id).max().unwrap_or_default();
            let mut nodes = parse_base64_input(&resp)?;

            for node in &mut nodes {
                last_id = last_id.next();
                node.id = last_id;
                node.belongs_to = Some(matched.id);
            }

            config.nodes.append(&mut nodes);
            config.save()?;

            Ok(())
        } else {
            bail!("subscription not found: {}", self.id_or_name);
        }
    }
}
