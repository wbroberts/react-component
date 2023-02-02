use std::{error::Error, fs, io::Write, path::Path};

use tera::{Context, Tera};

pub fn get(template_name: &str, name: &str) -> Result<String, Box<dyn Error>> {
    let mut tera = Tera::new("src/templates/**/*.txt").unwrap();
    tera.add_template_file("src/templates/component.txt", None)?;
    tera.add_template_file("src/templates/component-test.txt", None)?;

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
