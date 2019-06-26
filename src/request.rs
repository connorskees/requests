use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{BufReader};
use std::net::{TcpStream, SocketAddr};

use regex::Regex;
use url::{Url, ParseError};

#[path = "utils/structures.rs"]
pub mod structures;

#[path = "response.rs"]
mod response;

#[path = "utils/macros.rs"]
#[macro_use]
mod macros;

pub struct Request {
    method: String,
    url: String,
    headers: structures::Headers,
    prepared_request: String,
    // body: String,
}

impl Request {
    pub fn new(method: String, url: String, headers: HashMap<String, String>/*, body: String*/) -> Result<Request, ParseError> {
        let mut headers = default_headers().update(headers);
        let parsed_url = Url::parse(&url)?;
        headers.insert(String::from("Host"), String::from(parsed_url.host_str().unwrap()));
        return Ok(Request {
            method: method,
            url: String::from(parsed_url.path()),
            headers: headers,
            prepared_request: String::from("")
            // body
        })
    }

    pub fn prepare(&mut self) {
        let req = format!(
            "{} {} HTTP/1.0\n{}\n\n",
            self.method.to_uppercase(),
            self.url,
            self.headers.as_string()
        );
        self.prepared_request = req;
    }

    pub fn send(&self) -> response::Response {
        let address = &SocketAddr::from(([127, 0, 0, 1], 3000));
        let mut stream = TcpStream::connect(address).expect("Couldn't connect");
        let result = stream.write(self.prepared_request.as_bytes());
        // let mut buffer = Vec::new();

        let f = BufReader::new(stream);

        let mut currently_parsing = &Parse::FirstLine;
        let mut response = response::Response::new_blank();

        for line in f.lines() {
            let this_line = line.unwrap();
            match currently_parsing {
                &Parse::FirstLine => {
                    let first_line = Regex::new(r"(?P<version>HTTP/1.\d+) (?P<reponse_status>\d{3}) (?P<reason>[^\r\n]+)").unwrap();
                    let caps = first_line.captures(&this_line).unwrap();
                    let version = String::from(caps.name("version").unwrap().as_str());
                    let reponse_status = String::from(caps.name("reponse_status").unwrap().as_str());
                    let reason = String::from(caps.name("reason").unwrap().as_str());
                    response.version = version;
                    response.reponse_status = reponse_status;
                    response.reason = reason;
                    currently_parsing = &Parse::Header;
                },
                &Parse::Header => {
                    if this_line == "" {
                        currently_parsing = &Parse::Body;
                        continue
                    }
                    let headers_regex = Regex::new(r"(?P<key>[^:]+):\s*(?P<value>[^\r\n]+)").unwrap();
                    if headers_regex.is_match(&this_line) {
                        let caps = headers_regex.captures(&this_line).unwrap();
                        let key = String::from(caps.name("key").unwrap().as_str());
                        let value = String::from(caps.name("value").unwrap().as_str());
                        response.headers.insert(key, value);
                    }
                },
                &Parse::Body => {
                    continue
                }
            }
        }
        response

        // let response = stream.read_to_end(&mut buffer);
        // println!("{:?}", std::str::from_utf8(&buffer).unwrap());
        // return self.parse_response(std::str::from_utf8(&buffer).unwrap());
    }
}

enum Parse {
    FirstLine,
    Header,
    Body
}

pub fn default_headers() -> structures::Headers{
        structures::Headers::from_hash(map!(
            "Encoding" => "gzip, deflate",
            "Accept" => "*/*",
            "User-Agent" => "RustRequests/0.0.1",
            "Connection" => "keep-alive"
        ))
    }