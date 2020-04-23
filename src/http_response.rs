#path = ["http_version.rs"] mod http_version;
use std::collections::HashMap;
use http_version::HttpVersion;
use http_version::http_version_string;

pub struct HttpResponse {
    http_version: HttpVersion,
    status_code: u16,
    status_msg: String,
    headers: HashMap<String, String>,
    body: String
}

impl HttpResponse {
    pub fn new() -> HttpResponse {
        HttpResponse {
            http_version: HttpVersion
        }
    }
}