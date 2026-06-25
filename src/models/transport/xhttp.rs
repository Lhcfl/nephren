use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// HttpObject 对应传输配置的 httpSettings 项（HTTP/2）
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    pub host: Option<String>,
    pub path: Option<String>,
    pub mode: Option<Mode>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "packet-up")]
    PacketUp,
    #[serde(rename = "stream-up")]
    StreamUp,
    #[serde(rename = "stream-one")]
    StreamOne,
}
