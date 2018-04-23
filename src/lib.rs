extern crate docker;

pub fn string_return() -> String {
    let s = String::from("a mutable string");
    s
}

pub fn get_default_client() -> docker::client::Client {
    let c = docker::client::Client { connection: "/var/run/docker.sock".to_string() };
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_string_test() {
        let t = string_return();
        assert_eq!(t, "a mutable string");
    }

    // #[test]
    // fn a_client_test() {
    //     let t = string_return();
    //     assert_eq!(t, "a mutable string");
    // }
}
