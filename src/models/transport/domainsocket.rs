use serde::{Deserialize, Serialize};

/// DomainSocketObject 对应传输配置的 dsSettings 项
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    pub path: String,
    #[serde(rename = "abstract", skip_serializing_if = "Option::is_none")]
    pub abstract_: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<bool>,
}
