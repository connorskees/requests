#![allow(dead_code)]

use request::Request;

#[path = "utils/structures.rs"]
mod structures;
mod request;

fn main() {
    let headers = map!(
        "Host" => "stackoverflow.com",
        "Accept" => "*/*"
    );
    let mut request = Request::new(String::from("GET"), String::from("http:/127.0.0.1:3000/"), headers).unwrap();
    request.prepare();
    print!("{}", request.send());
}