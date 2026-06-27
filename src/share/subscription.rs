use anyhow::Context;
use base64::prelude::*;
use log::error;

use crate::{models::node::Node, share::node};

pub fn parse_base64_input(input: &str) -> anyhow::Result<(Vec<Node>, usize)> {
    let decoded = BASE64_STANDARD
        .decode(input)
        .context("is not a valid base64")?;
    let decoded_str = String::from_utf8(decoded)?;

    let mut ret = vec![];
    let mut failed = 0;

    for (src, node) in decoded_str
        .lines()
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .map(|src| (src, node::parse(src)))
    {
        match node {
            Ok(node) => ret.push(node),
            Err(e) => {
                error!("failed to parse link: {e}\n\t note: the link is {src}");
                failed += 1;
            }
        }
    }

    Ok((ret, failed))
}
