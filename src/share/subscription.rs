use base64::prelude::*;
use log::error;

use crate::models::node::Node;

pub fn parse_base64_input(input: &str) -> anyhow::Result<(Vec<Node>, usize)> {
    let decoded = BASE64_STANDARD.decode(input)?;
    let decoded_str = String::from_utf8(decoded)?;

    let mut ret = vec![];
    let mut failed = 0;

    for links in decoded_str
        .lines()
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .map(parse_link)
    {
        match links {
            Ok(node) => ret.push(node),
            Err(e) => {
                error!("failed to parse link: {}", e);
                failed += 1;
            }
        }
    }

    Ok((ret, failed))
}

pub fn parse_link(input: &str) -> anyhow::Result<Node> {
    let input = input.trim();
    if input.starts_with("vmess://") {
        anyhow::bail!("unsupported link type: {}", input);
        // parse_vmess_link(input)
    } else {
        anyhow::bail!("unsupported link type: {}", input);
    }
}
