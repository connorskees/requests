use std::io::prelude::*;
use std::net::{TcpStream, SocketAddr};

#[path = "utils/structures.rs"]
pub mod structures;

#[path = "response.rs"]
mod response;

pub struct Request {
    method: String,
    url: String,
    headers: structures::Headers,
    // body: String,
}

impl Request {
    pub fn new(method: String, url: String, headers: structures::Headers/*, body: String*/) -> Request {
        return Request {
            method: method,
            url: url,
            headers: headers,
            // body
        }
    }

    pub fn prepare(self) -> String {
        format!(
            "{} {} HTTP/1.0\n{}\n\n",
            self.method.to_uppercase(),
            self.url,
            self.headers.key_value_as_string().join("\n")
        )
    }

    // pub fn send(self, prepared_request: String) -> response::Response {
    //     let address = &SocketAddr::from(([127, 0, 0, 1], 3000));
    //     let mut stream = TcpStream::connect(address).expect("Couldn't connect");
    //     let result = stream.write(request.prepare().as_bytes());
    //     println!("{}", result.unwrap());
    //     let mut buffer = Vec::new();
        
    //     let response = stream.read_to_end(&mut buffer);
    //     println!("{:?}", std::str::from_utf8(&buffer).unwrap());
    //     Ok(())
    // }
}