extern crate docker;

pub fn get_default_client() -> docker::client::Client {
    let c = docker::client::Client { connection: "/var/run/docker.sock".to_string() };
    c
}

// pub fn connect_client() {
// docker::client::Client::connect_default();
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_client_test() {
        let c = get_default_client();
        assert_eq!(c.connection, "/var/run/docker.sock");
    }
}
