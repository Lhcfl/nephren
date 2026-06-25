use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

pub mod vless;
pub mod vmess;

/// 出站协议
#[derive(Debug, Serialize, Deserialize, Clone)]
#[enum_dispatch(ProtocolInfo)]
pub enum Protocol {
    VMess(vmess::Config),
    VLess(vless::Config),
}

#[enum_dispatch]
pub trait ProtocolInfo {
    fn address(&self) -> String;
}

impl Protocol {
    pub fn kind(self: &Self) -> &'static str {
        match self {
            Self::VLess(_) => "vless",
            Self::VMess(_) => "vmess",
        }
    }
}
