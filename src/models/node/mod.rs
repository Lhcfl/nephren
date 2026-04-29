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

    pub fn matches(&self, id_or_name: &str) -> bool {
        self.0.to_string() == id_or_name
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind_name")]
pub enum NodeKind {
    Unknown,
    VMess(vmess::VMess),
}

impl Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeKind::Unknown => write!(f, "unknown"),
            NodeKind::VMess(_) => write!(f, "vmess"),
        }
    }
}
