use anyhow::bail;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use url::Url;

pub mod vless;
pub mod vmess;

/// 出站协议
#[derive(Debug, Serialize, Deserialize, Clone)]
#[enum_dispatch(ProtocolInfo)]
pub enum Protocol {
    VMess(vmess::Config),
    VLess(vless::Config),
}

impl Protocol {
    pub fn name(&self) -> &'static str {
        match self {
            Self::VMess(_) => "vmess",
            Self::VLess(_) => "vless",
        }
    }

    pub fn from_url(url: &Url) -> anyhow::Result<Protocol> {
        match url.scheme() {
            "vmess" => bail!("not implemented"),
            "vless" => Ok(Self::VLess(vless::Config::parse_from_url(url)?)),
            x => bail!("unknown scheme: {x}"),
        }
    }
}

#[enum_dispatch]
pub trait ProtocolInfo {
    fn address(&self) -> String;
}
