use std::fs;
use std::path::Path;

use crate::metadata::*;
use crate::utils::to_crate_name;

// metadata files

pub fn config_metadata(path: &Path) {
  // .gitignore
  let gitignore_content = r#"/target
    "#;
  create_gitignore(path, gitignore_content);
}

// Cargo.toml

pub fn routes_cargo_toml(name: &str, fluxor_version:  &str) -> String {
    let package_name = to_crate_name(name);
    format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
fluxor = "{}"
"#,
            package_name, fluxor_version
        )
}

// 1. routes example
// src/main.rs

pub fn routes_main_rs(path: &Path) {
    let content = r##"use fluxor::prelude::*;

fn home(_req: Req, _params: Params) -> Reply {
    boxed(async {
        Ok(Response::builder()
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(Body::from("
            <a href='/'>Home</a> | <a href='/about'>About</a>
            <hr>
            <h1>Home Page</h1>
            "))
            .unwrap())
    })
}

fn about(_req: Req, _params: Params) -> Reply {
    boxed(async {
        Ok(Response::builder()
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(Body::from("
            <a href='/'>Home</a> | <a href='/about'>About</a>
            <hr>
            <h1>About Page</h1>
            "))
            .unwrap())
    })
}

fn api_message(_req: Req, _params: Params) -> Reply {
    boxed(async move {
       let json_response = format!(r#"{{"message": "👋 Hello, World!"}}"#);
        
        Ok(Response::builder()
            .header("Content-Type", "application/json")
            .body(Body::from(json_response))
            .unwrap())
    })
}

// Define the function to set up routes
fn setup_routes(app: &mut Fluxor) {
    app.route(GET, "/", home);                // Set the home route.
    app.route(GET, "/about", about);          // Set the about route.
    app.route(POST, "/api/msg", api_message); // Set the api message route.
    app.route(GET, "/http-client", serve_http_client); // A simple http client to test your application.
}

#[tokio::main]
async fn main() {
    let mut app = Fluxor::new();        // Initialize the application.
    setup_routes(&mut app);             // Setup HTTP routes.
    app.run("127.0.0.1", "8080").await; // Start the HTTP server (host, port).
}
"##;

    fs::write(path.join("main.rs"), content)
        .expect("Failed to create src/main.rs for routes-project example");
}

// 2. routes-projet example
// src/main.rs

pub fn routes_project_main_rs(name: &str, path: &Path) {
    let crate_name = to_crate_name(name);
    let content = format!(
        r#"use fluxor::prelude::*;

// project name / routes / setup_routes function
use {}::routes::setup_routes;

#[tokio::main]
async fn main() {{
    let mut app = Fluxor::new(); // Initialize the application.
    setup_routes(&mut app);              // Setup HTTP routes.
    app.run("127.0.0.1", "8080").await;  // Start the HTTP server (host, port).
}}
"#,
        crate_name
    );

    fs::write(path.join("main.rs"), content)
        .expect("Failed to create src/main.rs for routes-project example");
}

// src/lib.rs

pub fn routes_project_lib_rs(path: &Path) {
    let content = r#"pub mod routes;"#;

    fs::write(path.join("lib.rs"), content)
        .expect("Failed to create src/lib.rs for routes-project example");
}

// src/routes/mod.rs

pub fn routes_project_routes_mod_rs(path: &Path) {
    let content = r#"use fluxor::prelude::*;

// routes modules
mod pages;
mod api;

// Define the function to set up routes
pub fn setup_routes(app: &mut Fluxor) {
    app.route(GET, "/", pages::home::home);             // Home route
    app.route(GET, "/about", pages::about::about);      // About route.
    app.route(POST, "/api/msg", api::msg::api_message); // The api message route.
    app.route(GET, "/api/req/<id>", api::dynamic_route::dynamic_route_by_request);      // Request approach
    app.route(GET, "/api/params/<id>", api::dynamic_route::dynamic_route_by_params);    // Params approach
    app.route(GET, "/http-client", serve_http_client);  // A simple http client to test your application.
}"#;

    fs::write(path.join("mod.rs"), content)
        .expect("Failed to create src/routes/mod.rs for routes-project example");
}

// src/routes/api/mod.rs

pub fn routes_project_routes_api_mod_rs(path: &Path) {
    let content = r#"pub mod msg;
pub mod dynamic_route;"#;

    fs::write(path.join("mod.rs"), content)
        .expect("Failed to create src/routes/api/mod.rs for routes-project example");
}

// src/routes/api/msg.rs

pub fn routes_project_routes_api_msg_rs(path: &Path) {
    let content = r##"use fluxor::prelude::*;

pub fn api_message(_req: Req, _params: Params) -> Reply {
    boxed(async move {
       let json_response = format!(r#"{{"message": "👋 Hello, World!"}}"#);
        
        Ok(Response::builder()
            .header("Content-Type", "application/json")
            .body(Body::from(json_response))
            .unwrap())
    })
}
"##;

    fs::write(path.join("msg.rs"), content)
        .expect("Failed to create src/routes/api/msg.rs for routes-project example");
}

// src/routes/api/dynamic_route.rs

pub fn routes_project_routes_api_dynamic_route_rs(path: &Path) {
    let content = r###"use fluxor::prelude::*;

const JSON_TEMPLATE: &str = r##"{
  "id": "{{dynamic_id}}",
  "dynamic_route": "by_{{approach}}",
  "message": "The id value was retrieved using the {{approach}} approach."
}"##;

pub fn dynamic_route_by_request(req: Req, _params: Params) -> Reply {
    // app.route(GET, "/api/req/<id>", dynamic_route_by_request);
    // Clone the path string
    let path = req.uri().path().to_string();

    boxed(async move {
        // Use the cloned string inside async block
        let id_value = path.trim_start_matches("/api/req/");
        
        let json_response = do_json!(
            JSON_TEMPLATE,
            dynamic_id = id_value,
            approach = "request"
        );

        Ok(Response::builder()
            .header("Content-Type", "application/json")
            .body(Body::from(json_response))
            .unwrap())
    })
}

pub fn dynamic_route_by_params(_req: Req, params: Params) -> Reply {
    // app.route(GET, "/api/params/<id>", dynamic_route_by_params);
    // Retrieve the "id" parameter from params.extra
    let id_value = params.extra.get("id").cloned().unwrap_or_default();

    boxed(async move {
        let json_response = do_json!(
            JSON_TEMPLATE,
            dynamic_id = &id_value,
            approach = "params"
        );

        Ok(Response::builder()
            .header("Content-Type", "application/json")
            .body(Body::from(json_response))
            .unwrap())
    })
}"###;

    fs::write(path.join("dynamic_route.rs"), content)
        .expect("Failed to create src/routes/api/dynamic_route.rs for routes-project example");
}

// src/routes/pages/mod.rs

pub fn routes_project_routes_pages_mod_rs(path: &Path) {
    let content = r#"pub mod home;
pub mod about;"#;

    fs::write(path.join("mod.rs"), content)
        .expect("Failed to create src/routes/pages/mod.rs for routes-project example");
}

// src/routes/pages/home.rs

pub fn routes_project_routes_pages_home_rs(path: &Path) {
    let content = r#"use fluxor::prelude::*;

pub fn home(_req: Req, _params: Params) -> Reply {
    boxed(async {
        Ok(Response::builder()
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(Body::from("
            <a href='/'>Home</a> | <a href='/about'>About</a>
            <hr>
            <h1>Home Page</h1>
            "))
            .unwrap())
    })
}"#;

    fs::write(path.join("home.rs"), content)
        .expect("Failed to create src/routes/pages/home.rs for routes-project example");
}

// src/routes/pages/about.rs

pub fn routes_project_routes_pages_about_rs(path: &Path) {
    let content = r#"use fluxor::prelude::*;

pub fn about(_req: Req, _params: Params) -> Reply {
    boxed(async {
        Ok(Response::builder()
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(Body::from("
            <a href='/'>Home</a> | <a href='/about'>About</a>
            <hr>
            <h1>About Page</h1>
            "))
            .unwrap())
    })
}"#;

    fs::write(path.join("about.rs"), content)
        .expect("Failed to create src/routes/pages/about.rs for routes-project example");
}
