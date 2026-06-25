use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// HttpObject 对应传输配置的 httpSettings 项（HTTP/2）
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Vec<String>>>,
}
