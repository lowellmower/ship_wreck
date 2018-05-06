extern crate docker;
extern crate config;

pub fn get_default_client() -> docker::client::Client {
    let c = docker::client::Client { connection: "/var/run/docker.sock".to_string() };
    c
}

pub fn get_client_from_file(file: &str) -> docker::client::Client {
    let config = load_config(file);
    docker::client::Client { connection: config.connection }
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

fn load_config(file: &str) -> config::config::Config {
    let c = config::config::get_client_config(file);
    return c;
}
