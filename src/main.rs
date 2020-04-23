mod http_request;

fn main() {
    let mut req = http_request::HttpRequest::new();
    req.set_path(&"/path".to_string());
    req.add_header(&"Accept-Language".to_string(), &"en".to_string());
    println!("{}", req.to_string());
}
