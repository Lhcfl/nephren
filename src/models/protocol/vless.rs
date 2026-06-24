use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json;
use url::Url;

use crate::models::protocol::ProtocolInfo;

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
    #[default]
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "xtls-rprx-vision")]
    XtlsRprxVision,
    #[serde(rename = "xtls-rprx-vision-udp443")]
    XtlsRprxVisionUdp443,
}

impl Flow {
    pub fn is_empty(&self) -> bool {
        self == &Flow::Empty
    }
}

/// VLESS 是一个无状态的轻量传输协议，它分为入站和出站两部分，可以作为 Xray 客户端和服务器之间的桥梁。
/// 与 VMess 不同，VLESS 不依赖于系统时间，认证方式同样为 UUID。
///
/// see: https://xtls.github.io/config/outbounds/vless.html
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// 服务端地址，指向服务端，支持域名、IPv4、IPv6。
    pub address: String,
    /// 服务端端口，通常与服务端监听的端口相同。
    pub port: u16,
    pub id: String,
    pub encryption: String,
    #[serde(skip_serializing_if = "Flow::is_empty")]
    pub flow: Flow,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse: Option<ReverseConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReverseConfig {
    pub tag: String,
    pub sniffing: Option<serde_json::Value>,
}

impl ProtocolInfo for Config {
    fn address(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct VLessQueryParams {
    #[serde(default = "VLessQueryParams::default_encryption")]
    encryption: String,
}

impl VLessQueryParams {
    pub fn default_encryption() -> String {
        "none".to_owned()
    }
}

impl Config {
    pub fn parse_from_url(url: &Url) -> anyhow::Result<Config> {
        let q: VLessQueryParams = serde_qs::from_str(url.query().context("no query found")?)?;

        Ok(Config {
            address: url.host_str().context("no address found")?.to_owned(),
            port: url.port().unwrap_or(443),
            id: url.username().to_owned(),
            encryption: q.encryption,
            flow: Flow::Empty,
            level: None,
            reverse: None,
        })
    }
}
