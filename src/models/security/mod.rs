pub mod tls;

use serde::{Deserialize, Serialize};

/// 是否启用传输层加密
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum Security {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "tls")]
    Tls(tls::Config),
}
