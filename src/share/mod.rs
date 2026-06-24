use base64::prelude::*;
use log::debug;
use url::Url;

use crate::models::node::Node;

pub mod subscription;

pub fn parse(input: &str) -> anyhow::Result<()> {
    let is_url = input.contains("://");

    if is_url {
        let url = Url::parse(input)?;

        println!("\n=========== Parsed URL Information ============");
        println!("{url:#?}");

        let ret = Node::from_url(&url)?;
        println!("\n=========== Result ============");
        println!("{ret:#?}")
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
