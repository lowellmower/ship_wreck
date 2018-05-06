extern crate ship_wreck;
extern crate clap;

use clap::{Arg, App};

fn main() {
    let sw_args = App::new("Ship Wreck")
        .version("0.0.1")
        .author("Lowell M. <lowell.mower@gmail.com>")
        .about("Health monitoring for Docker containers")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let config_file = sw_args.value_of("config").unwrap_or("default.yml");
    let client = ship_wreck::get_client_from_file(config_file);
    println!("{:?}", client);
    println!("Connecting client...\n");
    client.connect();
}
