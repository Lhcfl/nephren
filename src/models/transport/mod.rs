use serde::{Deserialize, Serialize};

use crate::models::security::Security;

pub mod domainsocket;
pub mod grpc;
pub mod http;
pub mod kcp;
pub mod quic;
pub mod tcp;
pub mod ws;

/// Transport 包含网络类型及其对应的传输设置
///
/// 序列化为 adjacently tagged 格式: `{"network":"tcp","settings":{...}}`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "network", content = "settings")]
pub enum Transport {
    #[serde(rename = "tcp")]
    Tcp(tcp::Config),
    #[serde(rename = "kcp")]
    Kcp(kcp::Config),
    #[serde(rename = "ws")]
    Ws(ws::Config),
    #[serde(rename = "http")]
    Http(http::Config),
    #[serde(rename = "domainsocket")]
    DomainSocket(domainsocket::Config),
    #[serde(rename = "quic")]
    Quic(quic::Config),
    #[serde(rename = "grpc")]
    Grpc(grpc::Config),
}

impl Default for Transport {
    fn default() -> Self {
        Self::Tcp(tcp::Config::default())
    }
}

/// StreamSettings 对应出站入站协议中的 streamSettings 项
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StreamSettings {
    #[serde(flatten)]
    pub transport: Transport,
    #[serde(rename = "security")]
    pub security: Security,
}

/// KCP/QUIC 共用数据包头部伪装
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketHeader {
    #[serde(rename = "type")]
    pub type_: PacketHeaderType,
}

/// 伪装类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PacketHeaderType {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "srtp")]
    Srtp,
    #[serde(rename = "utp")]
    Utp,
    #[serde(rename = "wechat-video")]
    WechatVideo,
    #[serde(rename = "dtls")]
    Dtls,
    #[serde(rename = "wireguard")]
    Wireguard,
}
