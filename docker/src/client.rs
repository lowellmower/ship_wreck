use std::os::unix::net::UnixStream;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Client {
    pub connection: String,
}

impl Client {
    pub fn connect_default() {
        let mut stream = UnixStream::connect("/var/run/docker.sock").unwrap();
        stream.write_all(b"/containers/json").unwrap();

        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();

        println!("{}", response);
    }
}
