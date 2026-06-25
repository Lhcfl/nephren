use anyhow::Context;
use base64::prelude::*;
use log::{debug, error};
/// vless share link parser
use serde::{Deserialize, Serialize};
use url::Url;

use crate::models::{
    node::{Node, NodeId},
    protocol::{
        Protocol,
        vless::{Config, Flow},
        vmess::{self, Security},
    },
    transport::StreamSettings,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct VMessShare {
    ps: String,
    #[serde(rename = "add")]
    address: String,
    port: String,
    id: String,
    #[serde(rename = "aid")]
    alter_id: String,
    #[serde(rename = "scy")]
    security: vmess::Security,
}

pub fn parse_vmess_url(url: &Url) -> anyhow::Result<Node> {
    error!("todo: parse transport and security from url");
    let transport = StreamSettings::default();

    let input = url.domain().context("no domain")?;
    let decoded = BASE64_STANDARD.decode(input)?;
    let decoded_str = String::from_utf8(decoded)?;

    debug!("{}", decoded_str);

    let VMessShare {
        ps,
        address,
        port,
        id,
        alter_id,
        security,
    } = serde_json::from_str(&decoded_str)?;

    let config = vmess::Config {
        address,
        port: port.parse()?,
        id,
        alter_id: alter_id.parse()?,
        level: None,
        security,
    };

    Ok(Node {
        id: NodeId(0),
        name: ps,
        belongs_to: None,
        protocol: Protocol::VMess(config),
        transport,
    })
}
