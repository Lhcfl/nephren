use serde::{Deserialize, Serialize};

use crate::models::subscription::SubscriptionId;

mod vmess;

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NodeId(i32);

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    id: NodeId,
    name: String,
    /// server address, support domain name and IP address
    address: String,
    /// server port, required
    port: u16,
    /// subscription ID this node belongs to, optional
    belongs_to: Option<SubscriptionId>,
    kind: NodeKind,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NodeKind {
    Unknown,
    VMess(vmess::VMess),
}