use std::fmt::Display;

use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    pub id: SubscriptionId,
    pub name: String,
    pub url: String,
    pub enable_update: bool,
    pub description: String,
}
