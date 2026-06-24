use serde::{Deserialize, Serialize};

/// TLS 配置
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
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
