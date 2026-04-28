use serde::{Deserialize, Serialize, ser};

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
    id: String,
    /// 
    alter_id: String,
    security: Security,
    mux: bool,
    network: Network,
    network_type: String,
    host: String,
    path: String,
    v: String,
    tls: Option<String>,
    verify_cert: bool,   
}