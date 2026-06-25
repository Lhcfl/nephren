use anyhow::Context;
use log::error;
/// vless share link parser
use serde::{Deserialize, Serialize};
use url::Url;

use crate::models::{
    node::{Node, NodeId},
    protocol::{
        Protocol,
        vless::{Config, Flow},
    },
    transport::StreamSettings,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct VLessQueryParams {
    #[serde(default = "VLessQueryParams::default_encryption")]
    encryption: String,
}

impl VLessQueryParams {
    pub fn default_encryption() -> String {
        "none".to_owned()
    }
}

pub fn parse_vless_url(url: &Url) -> anyhow::Result<Node> {
    error!("todo: parse transport and security from url");
    let transport = StreamSettings::default();

    let q: VLessQueryParams = serde_qs::from_str(url.query().context("no query found")?)?;

    let name = percent_encoding::percent_decode_str(url.fragment().unwrap_or("unnamed"))
        .decode_utf8_lossy()
        .to_string();

    let config = Config {
        address: url.host_str().context("no address found")?.to_owned(),
        port: url.port().unwrap_or(443),
        id: url.username().to_owned(),
        encryption: q.encryption,
        flow: Flow::Empty,
        level: None,
        reverse: None,
    };

    Ok(Node {
        id: NodeId(0),
        name,
        belongs_to: None,
        protocol: Protocol::VLess(config),
        transport,
    })
}
