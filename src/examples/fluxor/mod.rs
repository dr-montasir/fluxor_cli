use std::fs;
use std::path::Path;

use cans::do_replace;

use crate::metadata::*;
use crate::utils::{LICENSE_APACHE_CONTENT, LICENSE_MIT_CONTENT, to_crate_name};
use crate::copy_folder_dir;

// metadata files

pub fn config_metadata(path: &Path) {
    // .gitignore
    let gitignore_content = r#"# Folders
/target

# Files
Cargo.lock"#;
    create_gitignore(path, gitignore_content);

  let env_content = r#"# here env.."#;

    create_env(path, env_content);

    create_license(path, "LICENSE-MIT", LICENSE_MIT_CONTENT);
    create_license(path, "LICENSE-APACHE", LICENSE_APACHE_CONTENT);
}

// Cargo.toml

pub fn fluxor_template_cargo_toml(name: &str, crator_version: &str, fluxor_version:  &str) -> String {
    let package_name = to_crate_name(name);
    format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
crator = "{}"
fluxor = "{}"
"#,
            package_name, crator_version, fluxor_version
        )
}

pub fn fluxor_template_main_rs(name: &str, path: &Path) {
    let crate_name = to_crate_name(name);
    let content = do_replace!(
        r##"use fluxor::prelude::*;

// routes
use {{cratename}}::{not_found_page, routes::setup_routes};
       
#[tokio::main]
async fn main() {
    let mut app = Fluxor::new();			     // Initialize the application.
    let static_dir = "src/assets".to_string();   // Retrieve the static directory "assets"

    app.set_dir(static_dir);            // Set directory for static files

    setup_routes(&mut app);             // Setup HTTP routes.

    // Set custom 404 handler
    app.set_custom_404(|content_type| {
        match content_type {
            "application/json" => do_json!(r#"{"error": {"code": 404, "message": "Not Found."}}"#,),
            "text/html" => not_found_page(),
            _ => do_text("404 Resource Not Found."),
        }
    });

    app.run("0.0.0.0", "10000").await;		// Start the HTTP server (host, port).
}"##,
        cratename = crate_name
    );

    fs::write(path.join("main.rs"), content)
        .expect("Failed to create src/main.rs for fluxor-template example");
}

// src/lib.rs

pub fn fluxor_template_lib_rs(path: &Path) {
    let content = r#"pub mod components;
pub mod routes;
// pub mod db;
// pub mod helpers;
// pub mod validators;

pub use components::*;
pub use routes::*;
// pub use db::*;
// pub use helpers::*;
// pub use validators::*;"#;

    fs::write(path.join("lib.rs"), content)
        .expect("Failed to create src/lib.rs for fluxor-template example");
}

// Copy and Write Folders

// copy fluxor example folder to src.

pub fn fluxor_copy_folder(source_path: &str, dest_path: &Path, folder_name: &str) {
    match copy_folder_dir(Path::new(&source_path), &dest_path.join(folder_name)) {
        Ok(_) => println!("Copied {} successfully.", folder_name),
        Err(e) => eprintln!("Error copying {}: {}", folder_name, e),
    }
}