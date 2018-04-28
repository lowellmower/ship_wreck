extern crate yaml_rust;

use self::yaml_rust::{YamlLoader, YamlEmitter};

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Config {
    file: String,
    command: String,
    socket: String,
}

fn load_file() -> String {
    let filename = "test.yml";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents).expect(
        "something went wrong reading the file",
    );

    return contents;
}

fn parse_yaml(s: &String) -> yaml_rust::yaml::Yaml {
    let docs = YamlLoader::load_from_str(&s).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    // Debug support
    println!("{:?}", doc);

    // Index access for map & array
    assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
    assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);

    // Chained key/array access is checked and won't panic,
    // return BadValue if they are not exist.
    // assert!(doc["INVALID_KEY"][100].is_badvalue());

    // Dump the YAML object
    // let mut out_str = String::new();
    // {
    //     let mut emitter = YamlEmitter::new(&mut out_str);
    //     emitter.dump(doc).unwrap(); // dump the YAML object to a String
    // }
    // println!("{}", out_str);
    doc.clone()
}

pub fn get_client_config() -> Config {
    let s = load_file();
    let yml = parse_yaml(&s);

    Config {
        file: yml["file"].as_str().unwrap().to_string(),
        socket: yml["socket"].as_str().unwrap().to_string(),
        command: yml["command"].as_str().unwrap().to_string(),
    }
}
