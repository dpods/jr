use std::fs;
use std::io::{self, Read};
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;
extern crate colored_json;
extern crate serde;

use clap::{Arg, App};
use serde_json::Value;
use serde::Serialize;

lazy_static!{
    static ref INDENT_LEVELS: HashMap<&'static str, &'static str> = [
        ("0", ""),
        ("1", " "),
        ("2", "  "),
        ("3", "   "),
        ("4", "    "),
        ("5", "     "),
        ("6", "      "),
        ("7", "       "),
        ("8", "        "),
    ].iter().copied().collect();
}

fn main() {
    let matches = App::new("JSON Reader")
        .version("1.0")
        .about("An easier way to read json files from the command line")
        .arg(Arg::with_name("input")
            .value_name("INPUT")
            .help("Sets the input file to use")
            .index(1))
        .arg(Arg::with_name("indent")
            .short("i")
            .long("indent")
            .value_name("INDENT_LEVEL")
            .help("Sets the indent level")
            .takes_value(true))
        .get_matches();

    let input_string: String = match matches.value_of("input") {
        Some(input_file) => {
            let json: String = match fs::read_to_string(input_file) {
                Ok(s)  => s,
                Err(_) => panic!("Cannot read from input file"),
            };
            json
        },
        None => {
            let mut stdin_buffer = String::new();
            io::stdin().read_to_string(&mut stdin_buffer)
                .expect("could not read from stdin");
            stdin_buffer
        }
    };

    let indent_level: &str = matches.value_of("indent").unwrap_or("4");
    let indent_string = match INDENT_LEVELS.get(indent_level) {
        Some(s) => s,
        None => panic!("Invalid indent level. Please choose a value from 0-8")
    };

    let json: Value = match serde_json::from_str(&input_string) {
        Ok(j) => j,
        Err(e) => {
            println!("Invalid JSON: {}", e);
            std::process::exit(1)
        }
    };

    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(indent_string.as_bytes());
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    json.serialize(&mut ser).unwrap();

    println!("{}", String::from_utf8(ser.into_inner()).unwrap().to_owned());
}
