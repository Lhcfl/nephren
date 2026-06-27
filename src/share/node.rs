use anyhow::bail;
use url::Url;

use crate::{
    models::node::Node,
    share::{vless::parse_vless_url, vmess::parse_vmess_url},
};

pub fn parse(input: &str) -> anyhow::Result<Node> {
    let is_url = input.contains("://");

    if is_url {
        let url = Url::parse(input)?;
        let node = match url.scheme() {
            "vless" => parse_vless_url(&url),
            "vmess" => parse_vmess_url(&url),
            x => bail!("no such scheme: {x}"),
        }?;

        return Ok(node);
    } else {
        bail!("not a url");
    }
}
