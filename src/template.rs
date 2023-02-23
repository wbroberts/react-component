use std::{error::Error, fs, io::Write, path::Path};

use colored::Colorize;
use serde::Serialize;
use tinytemplate::TinyTemplate;

const COMPONENT_TEMPLATE: &str = r#"import React from 'react';

export type {name}Props = \{};

export const {name}: React.ComponentType<{name}Props> = (\{}) => \{
  return (<div>{name} renders</div>);
};
"#;

const TEST_TEMPLATE: &str = r#"import \{ screen, render } from '@testing-library/react';

import \{ {name} } from './{name}.component';

describe('{name}', () => \{
  it('renders', () => \{
    render(<{name} />);
      expect(screen.getByText(/{name} renders/)).toBeDefined();
  });
});
"#;

const STORY_TEMPLATE: &str = r#"import \{ ComponentMeta, Story } from '@storybook/react';

import \{ {name}, {name}Props } from './{name}.component';

const story: ComponentMeta<typeof {name}> = \{
  title: '{name}'
};

export const {name}Story: Story<{name}Props> = (args) => \{
  return (<{name} \{...args} />)
};

export default story;
"#;

pub enum TemplateType {
    Component,
    Test,
    Storybook,
}

impl ToString for TemplateType {
    fn to_string(&self) -> String {
        match self {
            TemplateType::Component => "component".to_string(),
            TemplateType::Test => "test".to_string(),
            TemplateType::Storybook => "stories".to_string(),
        }
    }
}

#[derive(Serialize)]
struct Context {
    name: String,
}

pub struct Template<'a> {
    renderer: TinyTemplate<'a>,
    context: Context,
    templates: Vec<TemplateType>,
}

impl<'a> Template<'a> {
    pub fn new(name: &str, templates: Vec<TemplateType>) -> Result<Template, Box<dyn Error>> {
        let mut renderer = TinyTemplate::new();

        renderer.add_template("component", COMPONENT_TEMPLATE)?;
        renderer.add_template("test", TEST_TEMPLATE)?;
        renderer.add_template("stories", STORY_TEMPLATE)?;

        let context = Context {
            name: String::from(name),
        };

        Ok(Template {
            renderer,
            context,
            templates,
        })
    }

    pub fn create(&self, path: &Path, contained: bool) -> Result<(), Box<dyn Error>> {
        if !path.exists() {
            create_path(&path)?;
        }

        for template_type in &self.templates {
            let mut template_path = Path::new(path).join(&self.context.name);
            let template_name = template_type.to_string();

            let mut ext = template_name.clone();
            ext.push_str(".tsx");

            template_path.set_extension(ext);

            let template = &self.renderer.render(&template_name, &self.context)?;

            write(&template_path.as_path(), &template)?;

            println!("✔️ {} {}", "Created".green(), template_path.display());
        }

        if contained {
            let export = format!("export * from './{}.component';", &self.context.name);
            let index_path = &path.join("index.ts");

            write(index_path, &export)?;
            println!("✔️ {} {}", "Created".green(), index_path.display());
        }

        Ok(())
    }
}

fn create_path(path: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(path)?;
    Ok(())
}

fn write(path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}
