use anyhow::{Context, bail};
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
        vmess,
    },
    security,
    transport::{self, StreamSettings, Transport},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct VMessShare {
    v: String,
    ps: String,
    #[serde(rename = "add")]
    address: String,
    port: String,
    id: String,
    #[serde(rename = "aid")]
    alter_id: String,
    #[serde(rename = "scy")]
    vmess_security: vmess::Security,
    net: String,
    #[serde(rename = "type")]
    transport_security: String,
    host: String,
    path: String,
    tls: String,
    sni: String,
    alpn: String,
    #[serde(rename = "fp")]
    fingerprint: String,
    insecure: String,
}

pub fn parse_vmess_url(url: &Url) -> anyhow::Result<Node> {
    let input = url.domain().context("no domain")?;
    let decoded = BASE64_STANDARD.decode(input)?;
    let decoded_str = String::from_utf8(decoded)?;

    let VMessShare {
        v,
        ps,
        address,
        port,
        id,
        alter_id,
        vmess_security,
        net,
        transport_security,
        host,
        path,
        tls,
        sni,
        alpn,
        fingerprint: fp,
        insecure,
    } = serde_json::from_str(&decoded_str)?;

    if v != "2" {
        bail!("`v` should equals 2. the decoded str is {decoded_str}")
    }

    let transport = match net.as_str() {
        "tcp" => Transport::Tcp(transport::tcp::Config {
            accept_proxy_protocol: false,
            header: None,
        }),
        x => bail!("not implemented: {x} {decoded_str}"),
    };

    let security = match transport_security.as_str() {
        "none" => security::Security::None,
        x => bail!("not implemented: {x} {decoded_str}"),
    };

    let config = vmess::Config {
        address,
        port: port.parse()?,
        id,
        alter_id: alter_id.parse()?,
        level: None,
        security: vmess_security,
    };

    Ok(Node {
        id: NodeId(0),
        name: ps,
        belongs_to: None,
        protocol: Protocol::VMess(config),
        transport: StreamSettings {
            transport,
            security,
        },
    })
}
