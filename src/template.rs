use std::{error::Error, fs, io::Write, path::Path};

use colored::Colorize;
use tera::{Context, Tera};

const COMPONENT_TEMPLATE: &str = r#"import React from 'react';

export type {{ name }}Props = {};

export const {{name}}: React.ComponentType<{{name}}Props> = ({}) => {
  return (<div>{{name}} renders</div>);
};
"#;

const TEST_TEMPLATE: &str = r#"import { screen, render } from '@testing-library/react';

import { {{name}} } from './{{name}}.component';

describe('{{name}}', () => {
  it('renders', () => {
    render(<{{name}} />);
      expect(screen.getByText(/{{name}} renders/)).toBeDefined();
  });
});
"#;

const STORY_TEMPLATE: &str = r#"import { ComponentMeta, Story } from '@storybook/react';

import { {{name}}, {{name}}Props } from './{{name}}.component';

const story: ComponentMeta<typeof {{name}}> = {
  title: '{{name}}'
};

export const {{name}}Story: Story<{{name}}Props> = (args) => {
  return (<{{name}} {...args} />)
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

pub struct Template {
    name: String,
    tera: Tera,
    templates: Vec<TemplateType>,
}

impl Template {
    pub fn new(name: &str, templates: Vec<TemplateType>) -> Result<Template, Box<dyn Error>> {
        assert!(templates.len() > 0);

        let mut tera = Tera::new("*")?;

        tera.add_raw_template("component", COMPONENT_TEMPLATE)?;
        tera.add_raw_template("test", TEST_TEMPLATE)?;
        tera.add_raw_template("stories", STORY_TEMPLATE)?;

        Ok(Template {
            name: String::from(name),
            tera,
            templates,
        })
    }

    pub fn create(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        if !path.exists() {
            create_path(&path)?;
        }

        let mut context = Context::new();
        context.insert("name", &self.name);

        for template_type in &self.templates {
            let mut template_path = Path::new(path).join(&self.name);
            let template_name = template_type.to_string();

            let mut ext = template_name.clone();
            ext.push_str(".tsx");

            template_path.set_extension(ext);

            let template = &self.tera.render(&template_name, &context)?;

            self.write(&template_path.as_path(), &template)?;

            println!("✔️ {} {}", "Created".green(), template_path.display());
        }

        Ok(())
    }

    fn write(&self, path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }
}

fn create_path(path: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(path)?;
    Ok(())
}
