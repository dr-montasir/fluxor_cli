use std::fs;
use std::path::Path;

pub fn routes_cargo_toml(name: &str, fluxor_version:  &str) -> String {
    format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
fluxor = "{}"
"#,
            name, fluxor_version // Using `name` here to set the package name
        )
}

pub fn routes_template(src_path:  &Path) {
    fs::write(
        src_path.join("main.rs"),
        r##"use fluxor::prelude::*;

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
    app.route("/", GET, home);                // Set the home route.
    app.route("/about", GET, about);          // Set the about route.
    app.route("/api/msg", POST, api_message); // Set the api message route.
    app.route("/http-client", GET, serve_http_client); // A simple http client to test your application.
}

#[tokio::main]
async fn main() {
    let mut app = Fluxor::new();        // Initialize the application.
    setup_routes(&mut app);             // Setup HTTP routes.
    app.run("127.0.0.1", "8080").await; // Start the HTTP server (host, port).
}
"##,
    )
    .expect("Failed to create main.rs for helloworld");
}
