use serde::{Deserialize, Serialize};

/// DomainSocketObject 对应传输配置的 dsSettings 项
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    pub path: String,
    #[serde(rename = "abstract", default)]
    pub abstract_: bool,
    #[serde(default)]
    pub padding: bool,
}
