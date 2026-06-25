use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// WebSocketObject 对应传输配置的 wsSettings 项
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    #[serde(rename = "acceptProxyProtocol")]
    pub accept_proxy_protocol: bool,
    #[serde(default = "default_path")]
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(rename = "maxEarlyData")]
    pub max_early_data: Option<u32>,
    #[serde(rename = "useBrowserForwarding")]
    pub use_browser_forwarding: bool,
    #[serde(rename = "earlyDataHeaderName")]
    pub early_data_header_name: Option<String>,
}

fn default_path() -> String {
    "/".to_owned()
}
