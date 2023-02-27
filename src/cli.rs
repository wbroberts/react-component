use std::path::PathBuf;

use clap::{builder::ValueParser, Arg, ArgAction, Command};

#[derive(Debug)]
pub struct CLI {
    pub name: String,
    pub path: PathBuf,
    pub tests: bool,
    pub stories: bool,
    pub contain: bool,
}

impl CLI {
    pub fn parse() -> Self {
        let matches = command().get_matches();

        let name = matches.get_one::<String>("name").unwrap();
        let path = matches.get_one::<PathBuf>("path").unwrap();
        let no_tests = matches.get_one::<bool>("tests").unwrap();
        let no_storybook = matches.get_one::<bool>("storybook").unwrap();
        let contain = matches.get_one::<bool>("contain").unwrap();

        let path = if *contain {
            let contained_path = path.join(&name);
            contained_path
        } else {
            path.to_owned()
        };

        CLI {
            path,
            name: name.to_owned(),
            tests: !no_tests.to_owned(),
            stories: !no_storybook.to_owned(),
            contain: contain.to_owned(),
        }
    }
}

fn command() -> Command {
    Command::new("react-component")
        .about("Generates files that are common for React components")
        .version("0.2.5")
        .arg(name_arg())
        .arg(test_flag())
        .arg(stories_flag())
        .arg(path_flag())
        .arg(contain_flag())
}

fn name_arg() -> Arg {
    Arg::new("name")
        .help("Creates a react component, test, and story")
        .required(true)
        .action(ArgAction::Append)
        .value_parser(ValueParser::string())
}

fn path_flag() -> Arg {
    Arg::new("path")
        .long("path")
        .required(false)
        .default_value("src/components")
        .action(ArgAction::Set)
        .value_parser(ValueParser::path_buf())
        .help("Where the files should go")
}

fn test_flag() -> Arg {
    Arg::new("tests")
        .long("no-tests")
        .short('T')
        .required(false)
        .action(ArgAction::SetTrue)
        .value_parser(ValueParser::bool())
        .help("Skip adding a test")
}

fn stories_flag() -> Arg {
    Arg::new("storybook")
        .long("no-stories")
        .short('S')
        .required(false)
        .action(ArgAction::SetTrue)
        .value_parser(ValueParser::bool())
        .help("Skip adding a story")
}

fn contain_flag() -> Arg {
    Arg::new("contain")
        .long("contain")
        .short('c')
        .required(false)
        .action(ArgAction::SetTrue)
        .value_parser(ValueParser::bool())
        .help("Contain all files within a single directory with the same name as the component")
}
