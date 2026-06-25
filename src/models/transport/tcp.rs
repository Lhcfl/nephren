use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// TcpObject 对应传输配置的 tcpSettings 项
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    #[serde(rename = "acceptProxyProtocol", skip_serializing_if = "Option::is_none")]
    pub accept_proxy_protocol: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<TcpHeader>,
}

/// TCP 头部伪装设置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TcpHeader {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "http")]
    Http(Box<HttpHeader>),
}

/// HTTP 伪装配置，必须在对应的入站出站连接上同时配置，且内容必须一致
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HttpHeader {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<HttpRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<HttpResponse>,
}

/// HTTP 请求伪装
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HttpRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Vec<String>>>,
}

/// HTTP 响应伪装
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HttpResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, Vec<String>>>,
}
