use std::fmt::Display;

use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
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
