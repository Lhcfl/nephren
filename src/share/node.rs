use anyhow::bail;

use crate::{
    models::node::Node,
    share::{vless, vmess},
};

pub fn parse(input: &str) -> anyhow::Result<Node> {
    match input.split_once("://") {
        Some(("vless", _)) => vless::parse(input),
        Some(("vmess", _)) => vmess::parse(input),
        Some((scheme, _)) => bail!("unknown scheme: {scheme}"),
        None => bail!("not a url"),
    }
}
