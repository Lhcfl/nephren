use base64::{Engine, prelude::BASE64_STANDARD};
use log::info;
use serde::{Deserialize, Serialize};

use crate::models::node::{Node, NodeId, NodeKind, vmess::VMess};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VMessObject {
    pub v: String,
    pub ps: String,
    pub add: String,
    pub port: String,
    pub id: String,
    pub aid: String,
    #[serde(default = "defaults::scy")]
    pub scy: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(default = "defaults::net")]
    pub net: String,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub path: String,
    pub tls: String,
    #[serde(default)]
    pub verify_cert: bool,
    #[serde(default)]
    pub sni: String,
    #[serde(default)]
    pub alpn: String,
    #[serde(default = "defaults::insecure")]
    pub insecure: String,
}

mod defaults {
    pub fn scy() -> String {
        "auto".to_string()
    }
    pub fn insecure() -> String {
        "0".to_string()
    }
    pub fn net() -> String {
        "tcp".to_string()
    }
}

pub fn parse_vmess_link(link: &str) -> anyhow::Result<Node> {
    let Some(encoded) = link.strip_prefix("vmess://") else {
        anyhow::bail!("invalid vmess link: {}", link);
    };

    let decoded = BASE64_STANDARD.decode(encoded)?;
    let decoded_str = String::from_utf8(decoded)?;

    info!("decoded: {}", decoded_str);

    let VMessObject {
        v,
        ps,
        add,
        port,
        id,
        aid,
        scy,
        type_field,
        net,
        host,
        path,
        tls,
        verify_cert,
        ..
    } = serde_json::from_str(&decoded_str)?;

    Ok(Node {
        id: NodeId::default(),
        name: ps,
        address: add,
        port: port.parse()?,
        belongs_to: None,
        kind: NodeKind::VMess(VMess {
            id,
            alter_id: aid,
            security: scy.try_into()?,
            mux: false,
            network: net.try_into()?,
            network_type: type_field,
            host,
            path,
            v,
            tls: tls.into(),
            verify_cert,
        }),
    })
}
