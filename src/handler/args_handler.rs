use clap::{App, Arg};
use std::process;
use colored::*;

pub fn handle_arguments() {
    // Create an instance of App
    let matches = App::new("My Program")
        .arg(Arg::with_name("local")
            .short('h')
            .long("local")
            .takes_value(true)
            .multiple(true)
            .allow_hyphen_values(true)
            .help("local"))
        .arg(Arg::with_name("local-port")
            .short('p')
            .long("flag-b")
            .takes_value(true)
            .help("Flag B"))
        .arg(Arg::with_name("dest-host")
            .short('H')
            .long("dhost")
            .takes_value(true)
            .help("Destination Host"))
        .arg(Arg::with_name("dest-port")
            .short('P')
            .long("dport")
            .takes_value(true)
            .help("Destination Port"))
        .get_matches();
    // Check if the flags are present
}
