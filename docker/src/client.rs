use std::os::unix::net::*;
use std::io::prelude::*;

#[derive(Debug, Clone)]
pub struct Client {
    pub connection: String,
}

impl Client {
    pub fn connect_default(self) {
        let mut stream = UnixStream::connect(self.connection).unwrap();
        stream.write_all(b"GET /events HTTP/1.0\r\n\r\n").unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        println!("{}", response);
    }
}
