use base64::prelude::*;

use crate::models::node::{Node, NodeId, NodeKind, vmess::VMess};

pub fn parse_base64_input(input: &str) -> anyhow::Result<Vec<Node>> {
    let decoded = BASE64_STANDARD.decode(input)?;
    let decoded_str = String::from_utf8(decoded)?;
    let links = decoded_str
        .lines()
        .map(|line| parse_vmess_link(line))
        .collect::<Vec<_>>();
    Ok(vec![])
}

pub fn parse_vmess_link(link: &str) -> anyhow::Result<Node> {
    let Some(encoded) = link.strip_prefix("vmess://") else {
        anyhow::bail!("invalid vmess link: {}", link);
    };

    let decoded = BASE64_STANDARD.decode(encoded)?;
    let decoded_str = String::from_utf8(decoded)?;

    println!("decoded vmess config: {}", decoded_str);

    Ok(Node {
        id: NodeId::default(),
        name: "vmess node".to_string(),
        address: "example.com".to_string(),
        port: 12345,
        belongs_to: None,
        kind: NodeKind::Unknown,
    })
}
