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

static MAIN_TOML_FILE : &str = "mrq.toml";

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
                               .help("Filename(s) of specification(s) or path where all .md files are processed")
                               .required(false)
                               .multiple(true)
                               .index(2))
                          .arg(Arg::with_name("wildcard")
                               .short("w")
                               .long("wildcard")
                               .help("Search for *.md instead of looking for mrq.toml")
                               .required(false)
                               .takes_value(true)
                               .default_value("*.md"))
                          .get_matches();
    
    if matches.is_present("filename") {
        let path = matches.value_of("filename").unwrap();
        // Four options here:
        // 1. Specification file (e.g. path/to/spec.md)
        // 2. Top-level project file (e.g. path/to/non_standard.toml)
        // 3. Wildcard of specification files (e.g. /path/to/*.md)
        // 4. Path to directory without filename (search for top-level file or wildcard if set) 
        match matches.value_of("command").unwrap().as_ref() {
            "check" => checker::check_single_file(path, None),
            _ => println!("something else")
        }
    } else {
        if matches.occurrences_of("wildcard") == 1 {
            println!("{}", matches.value_of("wildcard").unwrap());
        } else {
            // mrq.toml in current working directory
            println!("mrq.toml");
        }
    }

    
}
