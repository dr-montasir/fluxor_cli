use std::fs;
use std::path::Path;

use crate::utils::to_crate_name;

pub fn hello_world_cargo_toml(name: &str, fluxor_version:  &str) -> String {
    let crate_name = to_crate_name(name);
    format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
fluxor = "{}"
"#,
            crate_name, fluxor_version // Using `name` here to set the package name
        )
}

// src/main.rs

pub fn hello_world_main_rs(path:  &Path) {
    fs::write(
        path.join("main.rs"),
        r#"use fluxor::prelude::*;

fn hello(_req: Req, _params: Params) -> Reply {
    boxed(async {
        Ok(Response::builder()
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(Body::from("<h1>👋 Hello, World!</h1>"))
            .unwrap())
    })
}

#[tokio::main]
async fn main() {
    let mut app = Fluxor::new();        // Initialize the application.
    app.route("/", GET, hello);         // Set the route (path, method, handler).
    app.run("127.0.0.1", "8080").await; // Start the HTTP server (host, port).
}
"#,
    )
    .expect("Failed to create main.rs for helloworld");
}

// src/main.rs

pub fn hello_world_api_main_rs(path:  &Path) {
    fs::write(
        path.join("main.rs"),
        r##"use fluxor::prelude::*;

fn hello(_req: Req, _params: Params) -> Reply {
    boxed(async move {
       let json_response = format!(r#"{{"message": "👋 Hello, World!"}}"#);
        
        Ok(Response::builder()
            .header("Content-Type", "application/json")
            .body(Body::from(json_response))
            .unwrap())
    })
}

#[tokio::main]
async fn main() {
    let mut app = Fluxor::new();                        // Initialize the application.
    app.route("/", GET, hello);                         // Set the route (path, method, handler).
    app.route("/http-client", GET, serve_http_client);  // A simple http client to test your application.
    app.run("127.0.0.1", "8080").await;                 // Start the HTTP server (host, port).
}
"##,
    )
    .expect("Failed to create main.rs for fluxor-project");
}

pub fn hello_world_api_server_cargo_toml(name: &str, fluxor_version:  &str) -> String {
    let crate_name = to_crate_name(name);
    format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
fluxor = "{}"

[[bin]]
name = "server"
path = "src/server.rs"
"#,
            crate_name, fluxor_version // Use `name` here to set the package name
        )
}

// src/server.rs

pub fn hello_world_api_server_rs(path:  &Path) {
    fs::write(
        path.join("server.rs"),
        r##"use fluxor::prelude::*;

fn hello(_req: Req, _params: Params) -> Reply {
    boxed(async move {
       let json_response = format!(r#"{{"message": "👋 Hello, World!"}}"#);
        
        Ok(Response::builder()
            .header("Content-Type", "application/json")
            .body(Body::from(json_response))
            .unwrap())
    })
}

#[tokio::main]
async fn main() {
    let mut server = Fluxor::new();                         // Initialize the application.
    server.route("/", POST, hello);                         // Set the route (path, method, handler).
    server.route("/http-client", GET, serve_http_client);   // A simple http client to test your application.
    server.run("127.0.0.1", "8080").await;                  // Start the HTTP server (host, port).
}
"##,
    )
    .expect("Failed to create server.rs for fluxor-project");
}