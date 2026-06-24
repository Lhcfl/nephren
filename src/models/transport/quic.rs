use serde::{Deserialize, Serialize};

use super::PacketHeader;

/// QuicObject 对应传输配置的 quicSettings 项
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    #[serde(rename = "security", default)]
    pub security: String,
    #[serde(default)]
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<PacketHeader>,
}
