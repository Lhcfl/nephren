use base64::prelude::*;
use log::debug;
use url::Url;

mod node;
pub mod subscription;
mod vless;
mod vmess;

pub fn parse(input: &str) -> anyhow::Result<()> {
    let is_url = input.contains("://");

    if is_url {
        let url = Url::parse(input)?;

        println!("\n=========== Parsed URL Information ============");
        println!("{url:#?}");

        let node = node::parse(input)?;
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
