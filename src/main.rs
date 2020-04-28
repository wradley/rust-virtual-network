mod http_request;

fn main() {
    let mut req = http_request::HttpRequest::new();
    req.path = String::from("/path");
    req.headers.insert(String::from("Accept_Language"), String::from("en"));

    println!("{}", req.to_string());
}
