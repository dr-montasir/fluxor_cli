use std::fs;
use std::path::Path;

use crate::metadata::*;
use crate::utils::to_crate_name;

// metadata files

pub fn config_redis_metadata(path: &Path) {
  // .gitignore
  let gitignore_content = r#"# Folders
/target

# Files
.env"#;

  create_gitignore(path, gitignore_content);

  
  let env_content = r#"# HOST
HOST=0.0.0.0

# PORT
PORT=8080
"#;

  create_env(path, env_content);
}

// Cargo.toml

pub fn db_cargo_toml(name: &str, fluxor_version:  &str) -> String {
    let crate_name = to_crate_name(name);
    format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
fluxor = "{}"
"#,
            crate_name, fluxor_version
        )
}

// main.rs

pub fn db_redis_main_rs(path: &Path) {
    let content = r##"use fluxor::prelude::*;

fn index(_req: Req, _params: Params) -> Reply {
    boxed(async move {
        let json_response = format!(
            r#"{{"host": "{}", "port": "{}"}}"#,
            env_var("HOST", "0.0.0.0"), // Load HOST from environment; default to "0.0.0.0" if not set or not in .env
            env_var("PORT", "10000")    // Load PORT from environment; default to "10000" if not set or not in .env
        );
        
        Ok(Response::builder()
            .header("Content-Type", "application/json")
            .body(Body::from(json_response))
            .unwrap())
    })
}

#[tokio::main]
async fn main() {
    load_dotenv(); // Load environment variables from the .env file

    let mut app = Fluxor::new();        // Initialize the application

    app.route(GET, "/", index);         // Set the index route

    // Get server configuration from environment variables
    let port = env_var("PORT", "10000");   // PORT is loaded from .env if available; defaults to "10000" otherwise

    let host = env_var("HOST", "0.0.0.0"); // HOST is loaded from .env if available; defaults to "0.0.0.0" otherwise

    app.run(&host, &port).await; // Start the HTTP server with specified host and port
}
"##;

    fs::write(path.join("main.rs"), content)
        .expect("Failed to create src/main.rs for db-redis example");
}
