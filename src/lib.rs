extern crate docker;
extern crate config;

pub fn get_default_client() -> docker::client::Client {
    let c = docker::client::Client { connection: "/var/run/docker.sock".to_string() };
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_client_test() {
        let c = get_default_client();
        assert_eq!(c.connection, "/var/run/docker.sock");
    }
}

pub fn load_config() -> config::config::Config {
    // load file
    // parse attributes
    let c = config::config::get_client_config();
    println!("{:?}", c);
    c
    // configure client
}
