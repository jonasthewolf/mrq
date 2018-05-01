
extern crate clap;
extern crate regex;

use clap::{Arg, App};
use std::fs::File;
use std::io::prelude::*;
use regex::RegexBuilder;

// Add
//   Replace <new> with new ID
// Check
//   Well-formedness
//   Requirement duplicate
//   Consistency within different specs
//     Config Mgmt sagt was zusammengehört
//   Mandatory attributes set
//     Per Requirement
//     Per Document
// Remove
//   Remove requirement definition
//   Check requirement references
//   Check history?
// Impact
// Create Diff Report
// Create Report
//   Including traceability
// Renumerate
// Enumerate
//   Especially helpful for e.g. detailed design (first time use)
// Upgrade config
fn check(filename : &str) {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let re = RegexBuilder::new(r"(\{[A-Z]+\-[0-9]+\s+.*\n*.*\n*\})")
                .multi_line(true)
                .dot_matches_new_line(true)
                .build()
                .unwrap();
    for x in re.captures_iter(&contents) {
        println!("{:?}", x);
    }
}

fn main() {
    let matches = App::new("Markdown Requirements Management Tool")
                          .version("1.0")
                          .author("Jonas Wolf <jonas.wolf@gmx.eu>")
                          .about("Does awesome things")
                          .arg(Arg::with_name("command")
                               .help("Command")
                               .possible_values(&["add", "check", "remove", "impact", "diff", "report", "renumerate", "enumerate", "upgrade"])
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("filename")
                               .help("")
                               .required(false)
                               .index(2))
                          .get_matches();
    match matches.value_of("command").unwrap().as_ref() {
        "check" => check(matches.value_of("filename").unwrap()),
        _ => println!("something else")
    }
    println!("Hello, world!");
}
