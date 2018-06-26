extern crate codespan;
extern crate codespan_reporting;
extern crate regex;

use self::regex::Regex;
use codespan::{ByteIndex, CodeMap, Span};
use codespan_reporting::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::{emit, Diagnostic, Label, Severity};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub struct ProjectFileContext {
    _config: String,
}

struct SpecificationContext<'t> {
    req_prefix: Option<self::regex::Match<'t>>,
    _config: Option<String>
}

impl<'t> SpecificationContext<'t> {
    pub fn new() -> SpecificationContext<'t> {
        SpecificationContext {
            req_prefix: None,
            _config: None
        }
    }
}

static _SPEC_ATTRIBUTE_REGEX: &str = r"";
static REQ_ID_REGEX: &str = r"(?P<reqid>(?P<idprefix>[A-Z]+[A-Z\-]+)(?P<reqnum>[0-9]+))";
static REQ_TITLE_REGEX: &str = r"(?P<reqtitle>.*)";
static REQ_TEXT_REGEX: &str = r"(?P<reqtext>.*\n*.*\n*)";

pub fn check_inputs(inputs: Vec<PathBuf>, _project_inputs: Option<PathBuf>) {
    //                project::process_project_file(MAIN_TOML_FILE);
    inputs.into_iter().for_each(|x| check_single_file(&x, None));
}

pub fn check_single_file(filename: &PathBuf, _parent: Option<ProjectFileContext>) {
    let reg_ex: &str = &format!(
        r"(?msU)(\{{\s*{}\s+{}\s*\n+{}\}})",
        REQ_ID_REGEX, REQ_TITLE_REGEX, REQ_TEXT_REGEX
    );

    // TODO Inherit configuration from parent

    let mut f = File::open(&filename).expect("File not found.");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file.");

    let mut code_map = CodeMap::new();
    code_map.add_filemap(
        codespan::FileName::Real(filename.to_path_buf()),
        contents.clone(),
    );

    let mut context = SpecificationContext::new();

    // TODO Improve and extract to file
    let re = Regex::new(reg_ex).unwrap();

    let mut error = 0u32;

    // TODO Refactor to visitor pattern

    // Check for inconsistent prefix
    context.req_prefix = Some(re.captures(&contents).unwrap().get(1).unwrap());
    for x in re.captures_iter(&contents) {
        if x["idprefix"] != context.req_prefix.unwrap().as_str().to_owned() {
            error += 1;
            let xmatch = x.get(1).unwrap();
            let span = Span::new(ByteIndex(xmatch.start() as u32 + 1u32), 
                                ByteIndex(xmatch.end() as u32 + 1u32));
            let error = Diagnostic::new(Severity::Error, "Prefix is not consistent within file")
                .with_label(
                    Label::new_primary(span)
                        .with_message("This is the inconsistent prefix"),
                );
                // .with_label(
                //     Label::new_secondary(Span::from_offset(str_start, 2.into()))
                //         .with_message("This is the expected prefix"),
                // );
            let writer = StandardStream::stderr(ColorChoice::Auto);
            emit(&mut writer.lock(), &code_map, &error).unwrap();
            //println!("Error: Prefix is not consistent within file: Requirement {:?} [line:{:?}] should start with {:?}.", &x["reqid"], context.find_line(&x.get(1).unwrap()), context.req_prefix);
        }
    }

    // Check for duplicate IDs
    let mut numbers: HashMap<u32, u32> = HashMap::new();
    for x in re.captures_iter(&contents) {
        let this_num = x["reqnum"].parse::<u32>().unwrap();
        let e = numbers.entry(this_num).or_insert(0);
        *e += 1;
    }
    for (id, count) in numbers {
        if count > 1 {
            println!("Error: Duplicate Requirement ID: Requirement with ID \"{}{}\" occurs with count {}.", context.req_prefix.unwrap().as_str(), id, count);
            error += 1;
        }
    }

    if error == 0 {
        println!("All fine");
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;
    #[test]
    fn idprefix_inconsistent() {
        let output = Command::new("./target/debug/mrq")
            .args(&["check", "test/test_idprefix_inconsistent.md"])
            .output()
            .expect("failed to execute process");

        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "Error: Prefix is not consistent within file: "
        );
        //check_single_file("test/test_idprefix_inconsistent.md", None);
    }
    #[test]
    fn duplicate_id() {
        let output = Command::new("./target/debug/mrq")
            .args(&["check", "test/test_reqid_duplicate.md"])
            .output()
            .expect("failed to execute process");

        assert_eq!(
            String::from_utf8_lossy(&output.stderr),
            "Error: Prefix is not consistent within file: "
        );
        //check_single_file("test/test_idprefix_inconsistent.md", None);
    }
}
