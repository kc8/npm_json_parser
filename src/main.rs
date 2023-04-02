use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::process;

mod args;
mod parsing;

struct FileData {
    raw_data: String,
}

impl FileData {
    fn build(config: &args::ArgsConfig) -> Result<FileData, &'static String> {
        // TODO how do we return the Err up the call here?
        let contents = fs::read_to_string(&config.file_path).unwrap_or_else(|err| {
            eprint!("[ERROR]: {err}",);
            process::exit(1);
        });

        Ok(FileData { raw_data: contents })
    }
}

fn main() {
    let config = args::Cli::get_args();

    let raw_contents = FileData::build(&config).unwrap_or_else(|err| {
        eprint!("[ERROR] in reading File Data: {err}");
        process::exit(1);
    });

    let mut raw_package: Value =
        serde_json::from_str(&raw_contents.raw_data).unwrap_or_else(|err| {
            eprint!("[ERROR] in parsing JSON: {err}");
            process::exit(1);
        });
    let scripts = raw_package["scripts"].as_object_mut().unwrap();

    // valid: npm run "script"
    let mut test_first: String = String::from("");
    let mut script_names: HashSet<String> = HashSet::new();

    for x in scripts.iter_mut() {
        //println!("{}, {}", x.0, x.1);
        // TODO and NOTE we are stripping the quote here out of the json
        script_names.insert(x.0.replace('\"', ""));
        test_first += match x.1.as_str() {
            Some(s) => s,
            None => {
                eprint!("[ERROR]: Could not properly parse {}, check that the \"scripts\" JSON syntax is properly formatted", &config.file_path);
                process::exit(1);
            }
        };
        // Add space at the end of the string else words are not parsed correctly
        test_first += " ";
    }

    let clean_input = test_first.replace('\"', "");

    let mut parser = parsing::parser::Parser::new(&clean_input);

    while parser.current().token_type != parsing::tokens::END {
        match parser.current().token_type == parsing::tokens::NPM
            && parser.peek_next().token_type == parsing::tokens::RUN
        {
            true => {
                if !script_names.contains(parser.get_script_op().token_literal) {
                    if parser.get_script_op().token_literal == parsing::tokens::RUN
                        || parser.get_script_op().token_literal == parsing::tokens::NPM
                    {
                        let msg = format!(
                            "Syntax error in \"{} {} [INVALID COMMAND HERE]\"",
                            parser.current().token_literal,
                            parser.peek_next().token_literal,
                        );
                        display_error_and_exit(&msg);
                    }
                    let msg = format!(
                        "Unable to verify script \"{} {} {}\"",
                        parser.current().token_literal,
                        parser.peek_next().token_literal,
                        parser.get_script_op().token_literal,
                    );
                    display_error_and_exit(&msg);
                }
            }
            false => (),
        }
        parser.next();
    }
    eprint!("[SUCCESS]: All npm run scripts are okay");
}

fn display_error_and_exit(msg: &str) {
    eprint!("[ERROR]: {}", msg);
    process::exit(1);
}
