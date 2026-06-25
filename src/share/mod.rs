use anyhow::bail;
use base64::prelude::*;
use log::debug;
use url::Url;

use crate::{
    models::node::Node,
    share::{vless::parse_vless_url, vmess::parse_vmess_url},
};

pub mod subscription;
mod vless;
mod vmess;

pub fn parse(input: &str) -> anyhow::Result<()> {
    let is_url = input.contains("://");

    if is_url {
        let url = Url::parse(input)?;

        println!("\n=========== Parsed URL Information ============");
        println!("{url:#?}");

        let node = match url.scheme() {
            "vless" => parse_vless_url(&url),
            "vmess" => parse_vmess_url(&url),
            x => bail!("no such scheme: {x}"),
        };
        println!("\n=========== Result ============");
        println!("{node:#?}")
    } else {
        let text = parse_base64(input)?;
        println!("{text}");
    }

    Ok(())
}

pub fn parse_base64(input: &str) -> anyhow::Result<String> {
    let decoded = BASE64_STANDARD.decode(input)?;
    let decoded_str = String::from_utf8(decoded)?;

    debug!("base64 parsed: {decoded_str}");

    Ok(decoded_str)
}
