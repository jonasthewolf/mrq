extern crate codespan;
extern crate codespan_reporting;
extern crate regex;

use codespan::FileName::Real;
use self::regex::Regex;
use codespan::{ByteIndex, CodeMap, Span};
use codespan_reporting::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::{emit, Diagnostic, Label, Severity};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;


#[derive(Clone, Debug, Hash)]
struct ReqLocation<'r> {
    file: &'r PathBuf,
    id_start: usize,
    id_stop: usize,
    text_start: usize,
    text_stop: usize,
    all_start: usize,
    all_stop: usize
}

#[derive(Clone, Debug, Hash)]
struct Requirement<'t> {
    id_prefix: &'t str,
    title: &'t str,
    text: &'t str,
    id: u64,
    loc: ReqLocation<'t>
}

impl<'t> PartialEq for Requirement<'t> {
    fn eq(&self, other: &Requirement<'t>) -> bool {
        (self.id_prefix == other.id_prefix) && (self.id == other.id)
    }
}

impl<'t> Eq for Requirement<'t> {}

struct MrqAttribute<'a> {
    key: &'a str,
    value: &'a str
    // Location missing
}

struct MrqConfiguration {
    regex_id : String, 
    reqex_text : String, 
    regex_title : String,
    _SPEC_ATTRIBUTE_REGEX : String
}

impl MrqConfiguration {
    pub fn default() -> MrqConfiguration {
        MrqConfiguration {
            regex_id : r"(?P<reqid>(?P<idprefix>[A-Z]+[A-Z\-]+)(?P<reqnum>[0-9]+))".to_owned(),
            reqex_text : r"(?P<reqtext>.*\n*.*\n*)".to_owned(),
            regex_title : r"(?P<reqtitle>.*)".to_owned(),
            _SPEC_ATTRIBUTE_REGEX : r"".to_owned()
        }
    }
}

struct RequirementContext<'c> {
    reqs: Vec<Requirement<'c>>,
    attr: Vec<MrqAttribute<'c>>,
    config: MrqConfiguration,
    code_map: CodeMap,
    specification_contents: String
}

pub struct ProjectFileContext {
    _config: String,
}

struct SpecificationContext<'t> {
    req_prefix: Option<Requirement<'t>>,
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

fn parse_specification(file: &PathBuf, context: &mut RequirementContext) -> Result<(), io::Error> {
    let reg_ex: &str = &format!(
        r"(?msU)(\{{\s*{}\s+{}\s*\n+{}\}})",
        REQ_ID_REGEX, REQ_TITLE_REGEX, REQ_TEXT_REGEX
    );
    let re = Regex::new(reg_ex).unwrap();
    
    let mut f = File::open(&file)?;
    context.specification_contents = String::new();
    f.read_to_string(&mut context.specification_contents)?;

    context.code_map.add_filemap(
        Real(file.to_path_buf()),
        context.specification_contents.clone(),
    );
    
    for x in re.captures_iter(&context.specification_contents) {
        let r  = Requirement {
            id_prefix : x.name("prefix").unwrap().as_str(),
            text : x.name("text").unwrap().as_str(),
            title : x.name("title").unwrap().as_str(),
            id : x.name("id").unwrap().as_str().parse::<u64>().unwrap(),
            loc : ReqLocation {
                file : &file.clone().to_path_buf(),
                all_start : x.iter().nth(0).unwrap().unwrap().start(),
                all_stop : x.iter().last().unwrap().unwrap().end(),
                id_start : x.name("id").unwrap().start(),
                id_stop : x.name("id").unwrap().end(),
                text_start : x.name("text").unwrap().start(),
                text_stop : x.name("text").unwrap().end()
            }
        };
        context.reqs.push(r);
    }

    Ok(())
}

pub fn check_inputs(inputs: Vec<PathBuf>, _project_inputs: Option<PathBuf>) {
    //                project::process_project_file(MAIN_TOML_FILE);
    inputs.into_iter().for_each(|x| check_single_file(&x, None));
}

pub fn check_single_file(filename: &PathBuf, _parent: Option<ProjectFileContext>) {
    // TODO Inherit configuration from parent
    let mut context = SpecificationContext::new();
    let mut req_context = RequirementContext {
            reqs : Vec::new(),
            attr : Vec::new(),
            config : MrqConfiguration::default(),
            code_map : CodeMap::new(),
            specification_contents : String::default()
    };
    parse_specification(&filename, &mut req_context);

    // TODO Improve and extract to file
    let mut error = 0u32;

    // TODO Refactor to visitor pattern

    // Check for inconsistent prefix
    context.req_prefix = Some(req_context.reqs.get(0).unwrap().clone()); // Some(re.captures(&contents).unwrap().name("idprefix").unwrap());
    let local_req_prefix = context.req_prefix.unwrap();
    for ref x in req_context.reqs {
        if x.id_prefix != local_req_prefix.id_prefix {
            error += 1;
            let span = Span::new(ByteIndex(x.loc.id_start as u32 + 1u32), 
                                ByteIndex(x.loc.id_stop as u32 + 1u32));
            let error = Diagnostic::new(Severity::Error, "Prefix is not consistent within file")
                .with_label(
                    Label::new_primary(span)
                        .with_message("This is the inconsistent prefix"),
                )
                .with_label(
                    Label::new_secondary(Span::new(ByteIndex(local_req_prefix.loc.id_start as u32 + 1u32), 
                                                   ByteIndex(local_req_prefix.loc.id_stop as u32 + 1u32)))
                        .with_message("This is the expected prefix"),
                );
            let writer = StandardStream::stderr(ColorChoice::Auto);
            emit(&mut writer.lock(), &req_context.code_map, &error).unwrap();
        }
    }

    // Check for duplicate IDs
    let mut numbers: HashMap<&Requirement, (u32, &Requirement)> = HashMap::new();
    for ref x in req_context.reqs {
        let e = numbers.entry(x).or_insert((0, &x));
        e.0 += 1;
    }
    for (ref req, (ref count, ref first_req)) in numbers.iter() {
        if *count > 1 {
            let span = Span::new(ByteIndex(req.loc.id_start as u32 + 1u32), 
                                ByteIndex(req.loc.id_stop as u32 + 1u32));
            let diag_error = Diagnostic::new(Severity::Error, "Duplicate requirement ID found")
                .with_label(
                    Label::new_primary(span)
                        .with_message("This is the location where the duplicate is was found"),
                )
                .with_label(
                    Label::new_secondary(Span::new(ByteIndex(first_req.loc.id_start as u32 + 1u32), 
                                                   ByteIndex(first_req.loc.id_stop as u32 + 1u32)))
                        .with_message("This is the location where the first was found"),
                );
            let writer = StandardStream::stderr(ColorChoice::Auto);
            emit(&mut writer.lock(), &req_context.code_map, &diag_error).unwrap();
            println!("Error: Duplicate Requirement ID: Requirement with ID \"{}{}\" occurs with count {}.", req.id_prefix, req.id, count);
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
