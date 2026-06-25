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
    security::{self, tls},
    transport::{self, StreamSettings, Transport, tcp, ws},
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
    kind: String,
    host: String,
    path: String,
    tls: String,
    sni: Option<String>,
    alpn: String,
    #[serde(rename = "fp")]
    fingerprint: Option<String>,
    insecure: String,
}

pub fn parse_vmess_url(url: &Url) -> anyhow::Result<Node> {
    let input = url.domain().context("no domain")?;
    let decoded = BASE64_STANDARD.decode(input)?;
    let decoded_str = String::from_utf8(decoded)?;

    debug!("decoded str: {decoded_str}");

    let VMessShare {
        v,
        ps,
        address,
        port,
        id,
        alter_id,
        vmess_security,
        net,
        kind,
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
        "tcp" => Transport::Tcp(tcp::Config {
            header: None,
            ..Default::default()
        }),
        "ws" => Transport::Ws(ws::Config {
            path,
            ..Default::default()
        }),
        x => bail!("not implemented: {x} {decoded_str}"),
    };

    let security = if tls == "" {
        security::Security::None
    } else if tls == "tls" {
        security::Security::Tls(tls::Config {
            server_name: Some(host),
            alpn: vec![alpn],
            allow_insecure: insecure != "0",
            disable_system_root: false,
            certificates: vec![],
            verify_client_certificate: false,
            pinned_peer_certificate_chain_sha256: None,
        })
    } else {
        bail!("not implemented: tls={tls} {decoded_str}")
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
