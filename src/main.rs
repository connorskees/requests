use std::io::prelude::*;
use std::net::{TcpStream, SocketAddr};

use request::Request;

#[path = "utils/structures.rs"]
mod structures;
mod request;
mod macros;

fn main() -> std::io::Result<()> {
    let mut headers = map!(
        "Host" => "localhost",
        "Accept" => "*/*"
    );
    let request = Request::new(String::from("GET"), String::from("/"), request::structures::Headers::from_hash(headers));
    // request.prepare();
    // println!("{:?}", )
    // let address = &SocketAddr::from(([127, 0, 0, 1], 3000));
    let mut stream = TcpStream::connect("stackoverflow.com:80").expect("Couldn't connect");
    let result = stream.write(request.prepare().as_bytes());
    println!("{}", result.unwrap());
    let mut buffer = Vec::new();
    
    let response = stream.read_to_end(&mut buffer);
    println!("{:?}", std::str::from_utf8(&buffer).unwrap());
    Ok(())
}

//127.0.0.1:3000