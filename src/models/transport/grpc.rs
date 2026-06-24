use serde::{Deserialize, Serialize};

/// grpcObject 对应传输配置的 grpcSettings 项
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    #[serde(rename = "serviceName")]
    pub service_name: String,
}
