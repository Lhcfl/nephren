use anyhow::bail;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Security {
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

impl TryFrom<String> for Security {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "auto" => Ok(Security::Auto),
            "none" => Ok(Security::None),
            "zero" => Ok(Security::Zero),
            "aes-128-gcm" => Ok(Security::Aes128Gcm),
            "chacha20-poly1305" => Ok(Security::Chacha20Poly1305),
            _ => bail!("bad security: {}", value), // return error if unknown
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Network {
    Tcp,
    Kcp,
    Ws,
    HttpUpgrade,
    XHttp,
    H2,
    Quic,
    Grpc,
}

impl TryFrom<String> for Network {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "tcp" => Ok(Network::Tcp),
            "kcp" => Ok(Network::Kcp),
            "ws" => Ok(Network::Ws),
            "http" => Ok(Network::HttpUpgrade),
            "xhttp" => Ok(Network::XHttp),
            "h2" => Ok(Network::H2),
            "quic" => Ok(Network::Quic),
            "grpc" => Ok(Network::Grpc),
            _ => bail!("bad network: {}", value), // return error if unknown
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VMess {
    /// user id, a 16-bytes random number, act as the token.
    pub id: String,
    pub alter_id: String,
    pub security: Security,
    pub mux: bool,
    pub network: Network,
    pub network_type: String,
    pub host: String,
    pub path: String,
    pub v: String,
    pub tls: Option<String>,
    pub verify_cert: bool,
}
