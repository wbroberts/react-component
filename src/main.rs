mod component;
mod template;

use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process,
};

use component::Component;

fn main() {
    let c = Component::get();

    if let Err(e) = check_or_create_path(&c.path) {
        println!("{e}");
        process::exit(1)
    }

    let mut name_path = PathBuf::from(&c.name.clone());

    if let Some(s) = name_path.extension() {
        eprintln!("error creating file because of an extension: {:?}", &s);
        process::exit(1);
    } else {
        name_path.set_extension("component.tsx");
    }

    if let Err(e) = create_component(&c.path.join(&name_path), &c.name) {
        println!("could not create file: {}", e);
        process::exit(1)
    }

    name_path.set_extension("test.tsx");

    if !c.skip_tests {
        if let Err(e) = create_component_test(&c.path.join(&name_path), &c.name) {
            println!("could not create file: {}", e);
            process::exit(1)
        }
    }
}

fn check_or_create_path(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let exists = Path::new(path).exists();

    if !exists {
        fs::create_dir_all(path)?;
    }

    Ok(())
}

fn create_component(path: &Path, name: &str) -> Result<(), Box<dyn Error>> {
    let content = template::get("component", &name).unwrap();
    template::create(&path, &content)?;

    Ok(())
}

fn create_component_test(path: &Path, name: &str) -> Result<(), Box<dyn Error>> {
    let content = template::get("test", &name).unwrap();
    template::create(&path, &content)?;

    Ok(())
}
