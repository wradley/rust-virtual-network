mod http_request;
mod serializable;

fn main() {
    let mut req = http_request::HttpRequest::new();
    req.set_path(&"/path".to_string());
    req.add_header(&"Accept-Language".to_string(), &"en".to_string());

    let serialized = serializable::to_bytes(&req);
    println!("{:#?}", serialized);
}
