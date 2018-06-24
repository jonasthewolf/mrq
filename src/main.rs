#[macro_use] extern crate log;
extern crate clap;
extern crate glob;

use clap::{Arg, App};
use std::path::{Path, PathBuf};
use glob::glob;

mod checker;
mod project;

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

    let mut inputs : Vec<PathBuf> = Vec::new();
    let mut project_file_input : Option<PathBuf> = None;
    // Four options here:
    // 1. Specification file (e.g. path/to/spec.md)
    // 2. Top-level project file (e.g. path/to/non_standard.toml)
    // 3. Wildcard of specification files (e.g. /path/to/*.md)
    // 4. Path to directory without filename (search for top-level file or wildcard if set) 
    if matches.is_present("filename") {
        let target_path = Path::new(matches.value_of("filename").unwrap());
        if target_path.is_file() {
            // Single file
            if target_path.extension().unwrap() == ".toml" {
                project_file_input = Some(target_path.to_path_buf());
            } else {
                inputs.push(target_path.to_path_buf());
            }
        } else {
            // Argument was wildcard, thus, only set of specifications possible
            let g = glob(target_path.to_str().unwrap()).expect("No input files found.");
            g.for_each(|x| inputs.push(x.unwrap().to_path_buf()));
        }
    } else {
        if matches.occurrences_of("wildcard") == 1 {
            let g = glob(matches.value_of("wildcard").unwrap()).expect("No input files found.");
            g.for_each(|x| inputs.push(x.unwrap().to_path_buf()));
        } else {
            // mrq.toml in current working directory
            let infile = format!("./{}", MAIN_TOML_FILE);
            project_file_input = Some(PathBuf::from(infile));
        }
    }

    match matches.value_of("command").unwrap().as_ref() {
        "check" => checker::check_inputs(inputs, project_file_input),
        _ => println!("something else")
    }
}
