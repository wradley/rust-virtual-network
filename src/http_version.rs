pub enum HttpVersion {
    HTTP0_9,
    HTTP1_0,
    HTTP1_1,
    HTTP2_0
}

pub fn http_version_string(method: &HttpVersion) -> &'static str {
    match method {
        HttpVersion::HTTP0_9 => "HTTP/0.9",
        HttpVersion::HTTP1_0 => "HTTP/1.0",
        HttpVersion::HTTP1_1 => "HTTP/1.1",
        HttpVersion::HTTP2_0 => "HTTP/2.0",
    }
}