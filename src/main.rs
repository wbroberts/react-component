mod cli;
mod template;

use std::process;

use cli::CLI;
use colored::Colorize;
use template::{Template, TemplateType};

fn main() {
    let args = CLI::parse();
    let templates = get_templates(&args);

    let template = match Template::new(&args.name, templates) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("❌ {}: {}", "Error".bold().red(), e);
            process::exit(1)
        }
    };

    let path = if args.contain {
        let contained_path = args.path.join(&args.name);
        contained_path
    } else {
        args.path
    };

    if let Err(e) = template.create(&path, args.contain) {
        eprintln!("❌ {}: {}", "Error".bold().red(), e);
        process::exit(1)
    }
}

fn get_templates(config: &CLI) -> Vec<TemplateType> {
    let mut templates: Vec<TemplateType> = Vec::with_capacity(3);

    templates.insert(0, TemplateType::Component);

    if config.tests {
        templates.insert(1, TemplateType::Test);
    }

    if config.stories {
        templates.insert(2, TemplateType::Storybook);
    }

    templates
}
