use std::fs;
use std::path::Path;

use crate::metadata::*;
use crate::utils::to_crate_name;

// metadata files

pub fn config_metadata(path: &Path) {
  // .gitignore
  let gitignore_content = r#"/target"#;

  create_gitignore(path, gitignore_content);
}

// Cargo.toml

pub fn template_cargo_toml(name: &str, fluxor_version:  &str) -> String {
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

pub fn template_main_rs(path: &Path) {
    let content = r##"use fluxor::prelude::*;
use fluxor::cans::html::*;
use fluxor::math::rand;

pub const HEAD: &str = r#"<head>
<meta charset="UTF-8">
    <title>{{page_title}} Page</title>
</head>"#;

pub const PAGES: [&str; 2] = [
    r#"<a href="/">Home</a>"#,
    r#"<a href="/about">About</a>"#
];

pub const HEADER: &str = r#"<header>
    {{navbar}}
</header>"#;

pub const STYLE: &str = r#"<style>
  header {
    background-color: #333;
    padding: 10px 20px;
  }
  header ul {
    list-style-type: none;
    margin: 0;
    padding: 0;
    display: flex;
  }
  header li {
    margin-right: 20px;
  }
  header a {
    color: white; 
    text-decoration: none;
    font-weight: bold;
  }
  header a:hover {
    text-decoration: underline;
  }
  h1 {
    font-family: Arial, sans-serif;
    color: #333;
  }
  h2 a {
    color: #0066cc;
    text-decoration: none;
  }
  h2 a:hover {
    text-decoration: underline;
  }
</style>"#;

pub const HOME_TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
    {{HEAD}}
    <body>
        {{STYLE}}
        {{HEADER}}
        <h1>Home Page</h1>
    </body>
</html>"#;

pub fn home(_req: Req, _params: Params) -> Reply {
    let home_template = do_html!(
        HOME_TEMPLATE,
        HEAD = HEAD,
        STYLE = STYLE,
        page_title = do_text("Home"), 
        HEADER = HEADER, 
        navbar =  do_forloop(&PAGES, 
            "<ul>", "<li>", "</li>", "</ul>"
        )
    );

    boxed(async {
        Ok(Response::builder()
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(Body::from(home_template))
            .unwrap())
    })
}

pub const ABOUT_TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
    {{HEAD}}
    <body>
        {{STYLE}}
        {{HEADER}}
        <h1>About Page</h1>
        {{component_if}}
    </body>
</html>"#;

pub fn about(_req: Req, _params: Params) -> Reply {
    let component_if: &str;
    let x = rand(1);

    if x == 1 {
        component_if = "<h2><a href='/{{x}}'>x = 1</a></h2>";
    } else if x > 1 && x < 6 {
        component_if = "<h2><a href=\"/{{x}}\">The variable 'x' is not equal to 1. It is within the range from 2 to 5. Therefore, 'x' is equal to ( {{x}} ).</a></h2>";
    } else {
        component_if = r#"<h2><a href="{{x}}">The variable 'x' is in the range from 6 to 9. Therefore, a randomly selected 'x' is equal to ( {{x}} ).</a></h2>"#;
    };

    let about_template = do_html!(
        ABOUT_TEMPLATE,
        HEAD = HEAD,
        STYLE = STYLE,
        page_title = "About",
        HEADER = HEADER,
        navbar =  do_forloop(&PAGES, "<ul>", "<li>", "</li>", "</ul>"),
        component_if = component_if,
        x = x // x must be defined after the component_if.
    );

    boxed(async {
        Ok(Response::builder()
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(Body::from(about_template))
            .unwrap())
    })
}

#[tokio::main]
async fn main() {
    let mut app = Fluxor::new();        // Initialize the application

    app.route("/", GET, home);          // Set the home route
    app.route("/about", GET, about);    // Set the about route

    app.run("127.0.0.1", "8080").await; // Start the HTTP server with specified host and port
}
"##;

    fs::write(path.join("main.rs"), content)
        .expect("Failed to create src/main.rs for routes-project example");
}
