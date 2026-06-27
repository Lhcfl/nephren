use anyhow::bail;
use log::info;

use crate::{
    models::{node::Node, subscription::Subscription},
    share::subscription::parse_base64_input,
};

impl Subscription {
    pub async fn pull(&self) -> anyhow::Result<(Vec<Node>, usize)> {
        let Some(url) = &self.url else {
            anyhow::bail!("subscription {} ({}) has no URL", self.id, self.name);
        };

        info!("pulling subscription {} ({})", self.id, self.name);

        let resp = reqwest::get(url).await?.text().await?;
        let (new_nodes, failed) = parse_base64_input(&resp)?;

        if new_nodes.is_empty() {
            bail!("no valid nodes found in subscription. {failed} nodes failed to parse.");
        }

        Ok((new_nodes, failed))
    }
}
