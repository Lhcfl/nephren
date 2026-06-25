use serde::{Deserialize, Serialize};

/// TLS 配置
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    /// 指定服务器端证书的域名，在连接由 IP 建立时有用。
    /// 当目标连接由域名指定时，比如在 Socks 入站时接收到了域名，或者由 Sniffing 功能探测出了域名，
    /// 这个域名会自动用于 serverName，无须手动配置。
    #[serde(rename = "serverName", skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// 一个字符串数组，指定了 TLS 握手时指定的 ALPN 数值。默认值为 ["h2", "http/1.1"]。
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alpn: Vec<String>,
    /// 是否允许不安全连接（仅用于客户端）。默认值为 false。
    /// 当值为 true 时，V2Ray 不会检查远端主机所提供的 TLS 证书的有效性。
    #[serde(rename = "allowInsecure")]
    pub allow_insecure: bool,
    /// （V2Ray 4.18+）是否禁用操作系统自带的 CA 证书。
    /// 默认值为 false。当值为 true 时，V2Ray 只会使用 certificates 中指定的证书进行 TLS 握手。
    /// 当值为 false 时，V2Ray 只会使用操作系统自带的 CA 证书进行 TLS 握手。
    #[serde(rename = "disableSystemRoot")]
    pub disable_system_root: bool,
    /// 证书列表，其中每一项表示一个证书（建议 fullchain）。
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub certificates: Vec<CertificateObject>,
    #[serde(rename = "verifyClientCertificate")]
    pub verify_client_certificate: bool,
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
    /// "encipherment"：证书用于 TLS 认证和加密。
    Encipherment,
    #[serde(rename = "verify")]
    /// "verify"：证书用于验证远端 TLS 的证书。当使用此项时，当前证书必须为 CA 证书。
    Verify,
    /// "issue"：证书用于签发其它证书。当使用此项时，当前证书必须为 CA 证书。
    #[serde(rename = "issue")]
    Issue,
    /// "verifyclient"：用于验证客户端身份的证书颁发机构证书。当使用此项时，当前证书必须为 CA 证书。 (4.42.0+)
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
