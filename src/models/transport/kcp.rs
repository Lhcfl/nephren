use serde::{Deserialize, Serialize};

use super::PacketHeader;

/// KcpObject 对应传输配置的 kcpSettings 项
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_mtu")]
    pub mtu: u32,
    #[serde(default = "default_tti")]
    pub tti: u32,
    #[serde(rename = "uplinkCapacity", default = "default_uplink_capacity")]
    pub uplink_capacity: u32,
    #[serde(rename = "downlinkCapacity", default = "default_downlink_capacity")]
    pub downlink_capacity: u32,
    #[serde(default)]
    pub congestion: bool,
    #[serde(rename = "readBufferSize", default = "default_buffer_size")]
    pub read_buffer_size: u32,
    #[serde(rename = "writeBufferSize", default = "default_buffer_size")]
    pub write_buffer_size: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<PacketHeader>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<String>,
}

fn default_mtu() -> u32 {
    1350
}
fn default_tti() -> u32 {
    50
}
fn default_uplink_capacity() -> u32 {
    5
}
fn default_downlink_capacity() -> u32 {
    20
}
fn default_buffer_size() -> u32 {
    2
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mtu: default_mtu(),
            tti: default_tti(),
            uplink_capacity: default_uplink_capacity(),
            downlink_capacity: default_downlink_capacity(),
            congestion: false,
            read_buffer_size: default_buffer_size(),
            write_buffer_size: default_buffer_size(),
            header: None,
            seed: None,
        }
    }
}
