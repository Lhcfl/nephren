/// VLESS 是一个无状态的轻量传输协议，它分为入站和出站两部分，可以作为 Xray 客户端和服务器之间的桥梁。
/// 与 VMess 不同，VLESS 不依赖于系统时间，认证方式同样为 UUID。
///
/// see: https://xtls.github.io/config/outbounds/vless.html

pub struct VLessConfig {
    address: String,
}
