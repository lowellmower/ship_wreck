extern crate yaml_rust;

use std::fs::File;
use std::io::prelude::*;

use self::yaml_rust::YamlLoader;


#[derive(Debug)]
pub struct Config {
    file: String,
    command: String,
    pub connection: String,
}

fn load_file(config: &str) -> String {
    let mut f = File::open(config).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect(
        "something went wrong reading the file",
    );

    return contents;
}

fn parse_yaml(s: &String) -> yaml_rust::yaml::Yaml {
    let docs = YamlLoader::load_from_str(&s).unwrap();
    return docs[0].clone();
}

pub fn get_client_config(file: &str) -> Config {
    let s = load_file(file);
    let yml = parse_yaml(&s);

    Config {
        file: yml["file"].as_str().unwrap().to_string(),
        connection: yml["connection"].as_str().unwrap().to_string(),
        command: yml["command"].as_str().unwrap().to_string(),
    }
}
