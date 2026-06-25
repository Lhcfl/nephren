use serde::{Deserialize, Serialize};

use crate::models::protocol::ProtocolInfo;

/// 加密方式，客户端将使用配置的加密方式发送数据，服务器端自动识别，无需配置。
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Security {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "zero")]
    Zero,
    #[serde(rename = "aes-128-gcm")]
    Aes128Gcm,
    #[serde(rename = "chacha20-poly1305")]
    Chacha20Poly1305,
}

/// VMess 是一个加密传输协议，它分为入站和出站两部分，通常作为 V2Ray 客户端和服务器之间的桥梁。
/// VMess 依赖于系统时间，请确保使用 V2Ray 的系统 UTC 时间误差在 90 秒之内，时区无关。
///
/// see: https://www.v2fly.org/config/protocols/vmess.html
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// 服务器地址，支持 IP 地址或者域名。
    pub address: String,
    /// 服务端端口，通常与服务端监听的端口相同。
    pub port: u16,
    /// VMess 用户的主 ID。必须是一个合法的 UUID。
    pub id: String,
    /// 为了进一步防止被探测，一个用户可以在主 ID 的基础上，再额外生成多个 ID。
    /// 这里只需要指定额外的 ID 的数量，推荐值为 0 代表启用 VMessAEAD。
    /// 不指定的话，默认值是 0。最大值 65535。这个值不能超过服务器端所指定的值。
    #[serde(default = "Config::default_alter_id")]
    #[serde(rename = "alterId")]
    pub alter_id: u16,
    /// 用户等级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    /// 加密方式，客户端将使用配置的加密方式发送数据，服务器端自动识别，无需配置。
    pub security: Security,
}

impl Config {
    pub fn default_alter_id() -> u16 {
        0
    }
}

impl ProtocolInfo for Config {
    fn address(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}
