use serde::{Deserialize, Serialize};

use super::PacketHeader;

/// QuicObject 对应传输配置的 quicSettings 项
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    #[serde(rename = "security", skip_serializing_if = "Option::is_none")]
    pub security: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<PacketHeader>,
}
