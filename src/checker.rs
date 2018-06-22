extern crate regex;


use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use self::regex::Regex;


pub struct ProjectFileContext {
    _config : String,
}

struct SpecificationContext {
    req_prefix : String,
    _config : Option<String>,
    new_lines : Vec<u32>,
}

impl SpecificationContext {
    pub fn new() -> SpecificationContext {
        SpecificationContext { 
                        req_prefix: "".to_string(), 
                        _config: None,
                        new_lines: Vec::new()
                      }
    }
    pub fn find_line(&self, what : &regex::Match) -> u32 {
        let x = what.start() as u32;
        self.new_lines.binary_search(&x).err().unwrap() as u32 + 1
    }
}

static _SPEC_ATTRIBUTE_REGEX : &str = r"";
static REQ_ID_REGEX : &str = r"(?P<reqid>(?P<idprefix>[A-Z]+[A-Z\-]+)(?P<reqnum>[0-9]+))";
static REQ_TITLE_REGEX : &str = r"(?P<reqtitle>.*)";
static REQ_TEXT_REGEX : &str = r"(?P<reqtext>.*\n*.*\n*)";

fn map_lines(context : &mut SpecificationContext, contents : &String) {
    let re = Regex::new(r"(\n)").unwrap();
    re.captures_iter(&contents).for_each( |m| { context.new_lines.push(m.get(1).unwrap().start() as u32 ) });
}

pub fn check_single_file(filename : &str, _parent : Option<ProjectFileContext>) {
    let reg_ex : &str = &format!(r"(?msU)(\{{\s*{}\s+{}\s*\n+{}\}})", REQ_ID_REGEX, REQ_TITLE_REGEX, REQ_TEXT_REGEX);

    let mut context = SpecificationContext::new();

    // Inherit configuration from parent

    let mut f = File::open(filename).expect("File not found.");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading the file.");
    // TODO Improve and extract to file
    let re = Regex::new(reg_ex).unwrap();

    map_lines(&mut context ,&contents);

    let mut error = 0u32;

    // TODO Refactor to visitor pattern

    // Check for inconsistent prefix
    context.req_prefix = re.captures(&contents).unwrap()["idprefix"].to_string();
    debug!("Prefix for spec {}", context.req_prefix);
    for x in re.captures_iter(&contents) {
        if x["idprefix"] != context.req_prefix {
            error += 1;
            error!("Error: Prefix is not consistent within file: Requirement {:?} [line:{:?}] should start with {:?}.", &x["reqid"], context.find_line(&x.get(1).unwrap()), context.req_prefix);
        }
    }

    // Check for duplicate IDs
    let mut numbers : HashMap<u32, u32> = HashMap::new();
    for x in re.captures_iter(&contents) {
        let this_num = x["reqnum"].parse::<u32>().unwrap();
        let e = numbers.entry(this_num).or_insert(0);
        *e += 1;
    }
    for (id, count) in numbers {
        if count > 1 {
            error!("Error: Duplicate Requirement ID: Requirement with ID \"{}{}\" occurs with count {}.", context.req_prefix, id, count);
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
    
        assert_eq!(String::from_utf8_lossy(&output.stderr), "Error: Prefix is not consistent within file: ");
        //check_single_file("test/test_idprefix_inconsistent.md", None);
    }
    #[test]
    fn duplicate_id() {
        let output = Command::new("./target/debug/mrq")
            .args(&["check", "test/test_reqid_duplicate.md"])
            .output()
            .expect("failed to execute process");
    
        assert_eq!(String::from_utf8_lossy(&output.stderr), "Error: Prefix is not consistent within file: ");
        //check_single_file("test/test_idprefix_inconsistent.md", None);
    }
}