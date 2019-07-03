use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{BufReader};
use std::net::{TcpStream};
use std::time::{Instant};

use native_tls::TlsConnector;
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
    pub method: &'static str,
    pub url: url::Url,
    pub headers: structures::Headers,
    pub prepared_request: String,
}

impl Request {
    pub fn new(method: &'static str, url: String, headers: HashMap<String, String>/*, body: String*/) -> Result<Request, ParseError> {
        let mut headers = default_headers().update(headers);
        let parsed_url = Url::parse(&url)?;
        let host = parsed_url.host_str().unwrap();
        headers.insert(String::from("Host"), String::from(host));
        return Ok(Request {
            method: method,
            url: parsed_url,
            headers: headers,
            prepared_request: String::from(""),
        })
    }

    pub fn prepare(&mut self) {
        let req = format!(
            "{} {} HTTP/1.1\r\n{}\r\n\r\n",
            self.method.to_uppercase(),
            self.url.path(),
            self.headers.as_string("\r\n")
        );
        self.prepared_request = req;
    }


    // fn handle_redirects(self) -> response::Response {

    // }

    pub fn read_response<T: std::io::Read>(self, stream: T) -> response::Response {
        let elapsed = Instant::now();

        let buf = BufReader::new(stream);

        let mut currently_parsing = &Parse::FirstLine;
        
        let mut version: String = String::from("");
        let mut response_status: String = String::from("");
        let mut reason: String = String::from("");
        let mut ok: bool = false;

        let mut should_redirect = false;

        let mut headers = response::structures::Headers::new();

        let mut body = std::string::String::new();

        for line in buf.lines() {
            let this_line = &line.unwrap();
            match currently_parsing {
                &Parse::FirstLine => {
                    let first_line = Regex::new(r"(?P<version>HTTP/1.\d+) (?P<response_status>\d{3}) (?P<reason>[^\r\n]+)").unwrap();
                    let caps = first_line.captures(this_line).expect("Missing first line");
                    version = String::from(caps.name("version").expect("Missing HTTP version").as_str());
                    response_status = String::from(caps.name("response_status").expect("Missing status code").as_str());
                    ok = response_status.starts_with("2");
                    should_redirect = response_status.starts_with("3");
                    reason = String::from(caps.name("reason").expect("Missing reason").as_str());
                    currently_parsing = &Parse::Header;
                },
                &Parse::Header => {
                    if this_line == "" {
                        currently_parsing = &Parse::Body;
                        continue;
                    }
                    let headers_regex = Regex::new(r"(?P<key>[^:]+):\s*(?P<value>[^\r\n]+)").unwrap();
                    if headers_regex.is_match(&this_line) {
                        let caps = headers_regex.captures(&this_line).unwrap();
                        let key = String::from(caps.name("key").unwrap().as_str());
                        let value = String::from(caps.name("value").unwrap().as_str());
                        headers.insert(key, value);
                    }
                },
                &Parse::Body => {
                    body = format!("{}\n{}", body, this_line);
                    continue;
                }
            }
        }

        println!("{:#?}", headers);


        if should_redirect {
            let location = String::from(headers.get(String::from("Location")).unwrap());
            let heads = self.headers.as_hash();
            println!("{:#?}", heads);
            let mut request = Request::new(self.method, location, heads).unwrap();
            request.prepare();
            return request.send();
        }

        else {
            response::Response {
                version: version,
                response_status: response_status,
                reason: reason,
                headers: headers,
                body: body,
                url: self.url,
                ok: ok,
                elapsed: elapsed.elapsed()
            }
        }
    }

    pub fn send(self) -> response::Response {
        let elapsed = Instant::now();
        let port: u16 = if self.url.scheme() == "https" { 443 } else { 80 };
        let host = self.url.host_str().unwrap();
        let address = &format!("{}:{}", host, port);
        match self.url.scheme() {
            "https" => {
                let connector = TlsConnector::new().expect("Unable to create TLS connector");
                let stream = TcpStream::connect(address).expect("Unable to connect to url");
                let mut stream = connector.connect(host, stream).expect("Couldn't connect");
                stream.write(self.prepared_request.as_bytes()).expect("Unable to send request");
                self.read_response(stream)
            },
            "http" | _ => {
                let mut stream = TcpStream::connect(address).expect(&format!("Couldn't connect to {}", address));
                stream.write(self.prepared_request.as_bytes()).expect("Unable to send request");
                self.read_response(stream)
            },
        }
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