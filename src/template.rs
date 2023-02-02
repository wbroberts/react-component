use std::{error::Error, fs, io::Write, path::Path};

use tera::{Context, Tera};

const COMPONENT_TEMPLATE: &str = r#"import React from 'react';

type {{ name }}Props = {};

export const {{name}}: React.ComponentType<{{name}}Props> = ({}) => {
  return (
    <div>{{name}} renders</div>
  )
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

pub fn get(template_name: &str, name: &str) -> Result<String, Box<dyn Error>> {
    let mut tera = Tera::new("./**/*").unwrap();
    tera.add_raw_template("component", COMPONENT_TEMPLATE)?;
    tera.add_raw_template("test", TEST_TEMPLATE)?;

    let mut context = Context::new();
    context.insert("name", &name);

    let template = tera.render(template_name, &context)?;

    Ok(template)
}

pub fn create(path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}
