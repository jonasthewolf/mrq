#[macro_use] extern crate log;
extern crate clap;


use clap::{Arg, App};


mod checker;

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
        "check" => checker::check_single_file(matches.value_of("filename").unwrap(), None),
        _ => println!("something else")
    }
    
}
