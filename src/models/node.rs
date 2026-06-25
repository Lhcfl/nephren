use crate::models::protocol;
use crate::models::security::Security;
use crate::models::subscription::SubscriptionId;
use crate::models::transport::StreamSettings;
use log::error;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use url::Url;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[serde(transparent)]
pub struct NodeId(pub i32);

impl Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl NodeId {
    pub fn next(self) -> NodeId {
        NodeId(self.0 + 1)
    }

    pub fn matches(&self, id_or_name: &str) -> bool {
        self.0.to_string() == id_or_name
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: NodeId,
    pub name: String,
    /// subscription ID this node belongs to, optional
    pub belongs_to: Option<SubscriptionId>,
    pub protocol: protocol::Protocol,
    pub transport: StreamSettings,
}
