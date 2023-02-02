use std::path::PathBuf;

use clap::{builder::ValueParser, Arg, ArgAction, Command};

#[derive(Debug)]
pub struct Component {
    pub name: String,
    pub path: PathBuf,
    pub skip_tests: bool,
}

impl Component {
    pub fn get() -> Self {
        let matches = command().get_matches();

        let name = matches.get_one::<String>("name").unwrap();
        let path = matches.get_one::<PathBuf>("path").unwrap();
        let skip_tests = matches.get_one::<bool>("skip-tests").unwrap();

        Component {
            name: name.to_owned(),
            path: path.to_owned(),
            skip_tests: skip_tests.to_owned(),
        }
    }
}

fn command() -> Command {
    Command::new("react-component")
        .arg(name_arg())
        .arg(path_flag())
        .arg(test_flag())
}

fn name_arg() -> Arg {
    Arg::new("name")
        .help("Creates a react component and test")
        .required(true)
        .action(ArgAction::Append)
        .value_parser(ValueParser::string())
}

fn path_flag() -> Arg {
    Arg::new("path")
        .long("path")
        .short('p')
        .required(false)
        .default_value("src/components")
        .action(ArgAction::Set)
        .value_parser(ValueParser::path_buf())
        .help("The path where the files should go")
}

fn test_flag() -> Arg {
    Arg::new("skip-tests")
        .long("skip-tests")
        .short('S')
        .required(false)
        .action(ArgAction::SetTrue)
        .value_parser(ValueParser::bool())
        .help("Skip adding the test file")
}
