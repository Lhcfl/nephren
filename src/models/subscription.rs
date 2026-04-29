use std::fmt::Display;

use anyhow::bail;
use log::info;
use serde::{Deserialize, Serialize};

use crate::{context::WithContext, inter::subscription::parse_base64_input, models::node::Node};

#[derive(
    Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash,
)]
#[serde(transparent)]
pub struct SubscriptionId(i32);

impl Display for SubscriptionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SubscriptionId {
    pub fn next(self) -> SubscriptionId {
        SubscriptionId(self.0 + 1)
    }

    pub fn matches(&self, query: &str) -> bool {
        self.0.to_string() == query
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub id: SubscriptionId,
    pub name: String,
    pub url: Option<String>,
    pub enable_update: bool,
    pub description: String,
}

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
