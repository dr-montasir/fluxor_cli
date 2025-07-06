use std::fs;
use std::path::Path;

pub fn create_gitignore(path: &Path, content: &'static str) {
    fs::write(path.join(".gitignore"), content)
        .expect("Failed to create .gitignore");
}

pub fn create_readme(path: &Path, crate_name: &str) {
    let content = format!(r#"# {}

This project has been initialized with the assistance of the [Fluxor CLI](https://crates.io/crates/fluxor_cli), a command-line tool that allows developers to quickly and efficiently create project starters for the [Fluxor web framework](https://crates.io/crates/fluxor)."
"#, crate_name);

    fs::write(path.join("README.md"), content)
        .expect("Failed to create README.md");
}

pub fn create_env(path: &Path, content: &'static str) {
    fs::write(path.join(".env"), content)
        .expect("Failed to create .env");
}