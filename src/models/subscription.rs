use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SubscriptionId(i32);

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscription {
    id: SubscriptionId,
    name: String,
    url: String,
    enable_update: bool,
    description: String,
}