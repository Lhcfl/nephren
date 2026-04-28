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
