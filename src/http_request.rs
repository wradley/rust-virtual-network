use std::collections::HashMap;
#[path = "http_version.rs"] mod http_version;
use http_version::HttpVersion;
use http_version::http_version_string;

pub enum HttpRequestMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    TRACE,
    OPTIONS,
    CONNECT,
    PATCH
}

pub fn http_request_method_string(method: &HttpRequestMethod) -> &'static str {
    match method {
        HttpRequestMethod::GET => "GET",
        HttpRequestMethod::HEAD => "HEAD",
        HttpRequestMethod::POST => "POST",
        HttpRequestMethod::PUT => "PUT",
        HttpRequestMethod::DELETE => "DELETE",
        HttpRequestMethod::TRACE => "TRACE",
        HttpRequestMethod::OPTIONS => "OPTIONS",
        HttpRequestMethod::CONNECT => "CONNECT",
        HttpRequestMethod::PATCH => "PATCH"
    }
}

pub struct HttpRequest {
    pub method: HttpRequestMethod,
    pub path: String,
    pub http_version: HttpVersion,
    pub headers: HashMap<String, String>,
    pub body: String
}

impl HttpRequest {

    pub fn new() -> HttpRequest {
        HttpRequest {
            method: HttpRequestMethod::GET,
            path: String::from("/"),
            http_version: HttpVersion::HTTP1_1,
            headers: HashMap::new(),
            body: String::from("")
        }
    }

    pub fn to_string(&self) -> String {
        let mut req = String::new();
        req.push_str(http_request_method_string(&self.method));
        req.push(' ');
        req.push_str(&self.path);
        req.push(' ');
        req.push_str(http_version_string(&self.http_version));
        req.push_str("\r\n");

        for (header, value) in &self.headers {
            req.push_str(header);
            req.push_str(": ");
            req.push_str(value);
            req.push_str("\r\n");
        }

        req.push_str(&self.body);
        req
    }
}