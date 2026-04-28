use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::models::subscription::SubscriptionId;

pub mod vmess;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
#[serde(transparent)]
pub struct NodeId(i32);

impl Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl NodeId {
    pub fn next(self) -> NodeId {
        NodeId(self.0 + 1)
    }

    pub fn matches(&self, query: &str) -> bool {
        self.0.to_string() == query
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub name: String,
    /// server address, support domain name and IP address
    pub address: String,
    /// server port, required
    pub port: u16,
    /// subscription ID this node belongs to, optional
    pub belongs_to: Option<SubscriptionId>,
    pub kind: NodeKind,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NodeKind {
    Unknown,
    VMess(vmess::VMess),
}
