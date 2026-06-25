use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// WebSocketObject 对应传输配置的 wsSettings 项
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(rename = "acceptProxyProtocol", skip_serializing_if = "Option::is_none")]
    pub accept_proxy_protocol: Option<bool>,
    #[serde(default = "default_path")]
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(rename = "maxEarlyData", skip_serializing_if = "Option::is_none")]
    pub max_early_data: Option<u32>,
    #[serde(rename = "useBrowserForwarding", skip_serializing_if = "Option::is_none")]
    pub use_browser_forwarding: Option<bool>,
    #[serde(rename = "earlyDataHeaderName", skip_serializing_if = "Option::is_none")]
    pub early_data_header_name: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            accept_proxy_protocol: None,
            path: default_path(),
            headers: None,
            max_early_data: None,
            use_browser_forwarding: None,
            early_data_header_name: None,
        }
    }
}

fn default_path() -> String {
    "/".to_owned()
}
