use anyhow::{Context, bail};
/// vless share link parser
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    models::{
        node::{Node, NodeId},
        protocol::{
            Protocol,
            vless::{Config, Flow},
        },
        security::{Security, tls},
        transport::{StreamSettings, Transport, xhttp},
    },
    share::vless::UrlShareTransport::XHttp,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct VLessQueryParams {
    #[serde(default = "VLessQueryParams::default_encryption")]
    encryption: String,
    flow: Flow,
    security: UrlShareSecurity,
    #[serde(rename = "type")]
    transport: UrlShareTransport,
    #[serde(rename = "pbk")]
    public_key: Option<String>,
    #[serde(rename = "sid")]
    short_id: Option<String>,
    sni: Option<String>,
    #[serde(rename = "fp")]
    fingerprint: Option<String>,
    path: Option<String>,
    mode: String,
    host: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum UrlShareSecurity {
    #[serde(rename = "")]
    None,
    #[serde(rename = "tls")]
    Tls,
    #[serde(rename = "reality")]
    Reality,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum UrlShareTransport {
    #[serde(rename = "xhttp")]
    XHttp,
}

impl VLessQueryParams {
    pub fn default_encryption() -> String {
        "none".to_owned()
    }
}

impl TryFrom<&VLessQueryParams> for Transport {
    type Error = anyhow::Error;
    fn try_from(value: &VLessQueryParams) -> anyhow::Result<Self> {
        Ok(match value.transport {
            XHttp => Transport::Xhttp(xhttp::Config {
                host: value.host.clone(),
                path: value.path.clone(),
                mode: serde_json::from_value(value.mode.clone().into())?,
                ..Default::default()
            }),
        })
    }
}

impl TryFrom<&VLessQueryParams> for Security {
    type Error = anyhow::Error;
    fn try_from(value: &VLessQueryParams) -> anyhow::Result<Self> {
        Ok(match value.security {
            UrlShareSecurity::None => Self::None,
            UrlShareSecurity::Tls => Self::Tls(tls::Config {
                server_name: value.sni.clone(),
                alpn: vec![],
                allow_insecure: false,
                disable_system_root: false,
                certificates: vec![],
                verify_client_certificate: false,
                pinned_peer_certificate_chain_sha256: None,
            }),
            UrlShareSecurity::Reality => bail!("reality is not implemented"),
        })
    }
}

pub fn parse_vless_url(url: &Url) -> anyhow::Result<Node> {
    let q: VLessQueryParams = serde_qs::from_str(url.query().context("no query found")?)?;

    let name = percent_encoding::percent_decode_str(url.fragment().unwrap_or("unnamed"))
        .decode_utf8_lossy()
        .to_string();

    let VLessQueryParams {
        encryption, flow, ..
    } = &q;

    let config = Config {
        address: url.host_str().context("no address found")?.to_owned(),
        port: url.port().unwrap_or(443),
        id: url.username().to_owned(),
        encryption: encryption.to_owned(),
        flow: *flow,
        level: None,
        reverse: None,
    };

    Ok(Node {
        id: NodeId(0),
        name,
        belongs_to: None,
        protocol: Protocol::VLess(config),
        transport: StreamSettings {
            transport: (&q).try_into()?,
            security: (&q).try_into()?,
        },
    })
}
