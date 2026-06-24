use serde::{Deserialize, Serialize};

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

/// 是否启用传输层加密
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Security {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "tls")]
    Tls,
}

/// StreamSettings 对应出站入站协议中的 streamSettings 项
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StreamSettings {
    #[serde(flatten)]
    pub transport: Transport,
    #[serde(rename = "security")]
    pub security: Security,
    #[serde(rename = "tlsSettings", skip_serializing_if = "Option::is_none")]
    pub tls_settings: Option<TlsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sockopt: Option<Sockopt>,
}

/// TLS 配置
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TlsConfig {
    #[serde(rename = "serverName", skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpn: Option<Vec<String>>,
    #[serde(rename = "allowInsecure", skip_serializing_if = "Option::is_none")]
    pub allow_insecure: Option<bool>,
    #[serde(rename = "disableSystemRoot", skip_serializing_if = "Option::is_none")]
    pub disable_system_root: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<CertificateObject>>,
    #[serde(
        rename = "verifyClientCertificate",
        skip_serializing_if = "Option::is_none"
    )]
    pub verify_client_certificate: Option<bool>,
    #[serde(
        rename = "pinnedPeerCertificateChainSha256",
        skip_serializing_if = "Option::is_none"
    )]
    pub pinned_peer_certificate_chain_sha256: Option<String>,
}

/// 证书用途
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CertificateUsage {
    #[default]
    #[serde(rename = "encipherment")]
    Encipherment,
    #[serde(rename = "verify")]
    Verify,
    #[serde(rename = "issue")]
    Issue,
    #[serde(rename = "verifyclient")]
    VerifyClient,
}

/// 证书对象
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CertificateObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<CertificateUsage>,
    #[serde(rename = "certificateFile", skip_serializing_if = "Option::is_none")]
    pub certificate_file: Option<String>,
    #[serde(rename = "keyFile", skip_serializing_if = "Option::is_none")]
    pub key_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<Vec<String>>,
}

/// 透明代理类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TproxyType {
    #[default]
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "redirect")]
    Redirect,
    #[serde(rename = "tproxy")]
    Tproxy,
}

/// Sockopt 透明代理配置
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Sockopt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark: Option<i32>,
    #[serde(rename = "tcpFastOpen", skip_serializing_if = "Option::is_none")]
    pub tcp_fast_open: Option<bool>,
    #[serde(
        rename = "tcpFastOpenQueueLength",
        skip_serializing_if = "Option::is_none"
    )]
    pub tcp_fast_open_queue_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tproxy: Option<TproxyType>,
    #[serde(
        rename = "tcpKeepAliveInterval",
        skip_serializing_if = "Option::is_none"
    )]
    pub tcp_keep_alive_interval: Option<i32>,
    #[serde(rename = "bindToDevice", skip_serializing_if = "Option::is_none")]
    pub bind_to_device: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mptcp: Option<bool>,
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
