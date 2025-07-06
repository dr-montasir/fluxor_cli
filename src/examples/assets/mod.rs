use std::fs;
use std::path::Path;

use crate::utils::to_crate_name;

// Cargo.toml

pub fn assets_cargo_toml(name: &str, fluxor_version:  &str) -> String {
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

pub fn assets_main_rs(path: &Path) {
    let content = r##"use fluxor::prelude::*;

const HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <link rel="stylesheet" href="/css/styles.css" />
        <title>Assets Example</title>
    </head>
    <body>
        <h1>
            <span class="hand">👋</span>
            <span class="animated-text" id="animatedText">&ensp;from&nbsp;the&nbsp;fluxor&nbsp;framework&ensp;</span>
        </h1>
        <img id="fluxor" src="/img/fluxor.svg" alt="Fluxor logo" />
        <script src="/js/script.js"></script>
    </body>
</html>"#;

fn home(_req: Req, _params: Params) -> Reply {
    boxed(async {
        Ok(Response::builder()
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(Body::from(HTML))
            .unwrap())
    })
}

#[tokio::main]
async fn main() {
    let mut app = Fluxor::new();        // Initialize the application.
    
    // Configure the application
    // let static_dir = "public".to_string();         // Retrieve the static directory "public"
    // let static_dir = "src/assets".to_string();     // Retrieve the static directory "src/assets"
    let static_dir = "assets".to_string();         // Retrieve the static directory "assets"

    app.set_dir(static_dir);            // Set directory for static files

    app.route("/", GET, home);          // Set the home route.
    app.run("127.0.0.1", "8080").await; // Start the HTTP server (host, port).
}
"##;

    fs::write(path.join("main.rs"), content)
        .expect("Failed to create src/main.rs for routes-project example");
}

// assets/css/styles.css

pub fn assets_css_styels_css(path: &Path) {
    let content = r#"body {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
  margin: 0;
  background-color: #874f40;
}

#fluxor {
  width: 100px;
  height: 100px;
  animation: expand-contract 4s infinite alternate ease-in-out;
}

@keyframes expand-contract {
  0% {
    width: 100px;
    height: 100px;
  }
  100% {
    width: 300px;
    height: 300px;
  }
}

.hand {
  display: inline-block;
  font-size: 2em;
  margin-left: 10px;
  animation: wave-hand 2s infinite;
  transform-origin: 70% 70%;
}

@keyframes wave-hand {
  0% { transform: rotate(0deg); }
  20% { transform: rotate(20deg); }
  40% { transform: rotate(-20deg); }
  60% { transform: rotate(20deg); }
  80% { transform: rotate(-20deg); }
  100% { transform: rotate(0deg); }
}

.animated-text span {
  border: 10px solid inherit;
  background-color: #61DAFB;
  color: #FF6D00;
  opacity: 0;
  display: inline-block;
  animation: fadeIn 9s infinite;
  border-radius: 25%;
  font-weight:bold;
  font-family: Verdana, Geneva, Tahoma, sans-serif;
  text-transform: uppercase;
}

@keyframes fadeIn {
  to {
    opacity: 1;
  }
}"#;

    fs::write(path.join("styles.css"), content)
        .expect("Failed to create assets/css/styles.css for assets example");
}

// assets/js/script.js

pub fn assets_js_script_js(path: &Path) {
    let content = r#"window.onload = () => {
  const fluxor = document.getElementById('fluxor');
  let expanding = true;
  const minSize = 100;
  const maxSize = 300;
  let size = minSize;

  setInterval(() => {
    if (expanding) {
      size += 10;
      if (size >= maxSize) {
        expanding = false;
      }
    } else {
      size -= 10;
      if (size <= minSize) {
        expanding = true;
      }
    }
    fluxor.style.width = size + 'px';
    fluxor.style.height = size + 'px';
  }, 100);

  const textContainer = document.getElementById('animatedText');
  const text = textContainer.textContent;
  textContainer.textContent = '';

  for (let i = 0; i < text.length; i++) {
    const span = document.createElement('span');
    span.textContent = text[i];
    span.style.animationDelay = `${i * 0.2}s`;
    textContainer.appendChild(span);
  }

  const spans = textContainer.querySelectorAll('span');
  spans.forEach(span => {
    span.style.animationName = 'fadeIn';
  });
};"#;

    fs::write(path.join("script.js"), content)
        .expect("Failed to create assets/js/script.js for assets example");
}

// assets/img/fluxor.svg

pub fn assets_img_fluxor_svg(path: &Path) {
    let content = r##"<svg width="64" height="64" viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
<path d="M32.4712 56.0684L24.0436 47.6408L47.6408 24.0436C52.2952 28.698 52.2952 36.2443 47.6408 40.8987L32.4712 56.0684Z" fill="#61DAFB"/>
<path d="M32.2667 35.6129L22.1536 45.726L18.7826 42.3549L28.8957 32.2418L32.2667 35.6129Z" fill="#FF6D00"/>
<path d="M32.0023 8.40495L40.4299 16.8325L16.8327 40.4298C12.1782 35.7753 12.1782 28.229 16.8327 23.5746L32.0023 8.40495Z" fill="#FF6D00"/>
<path d="M32.2419 28.8955L42.355 18.7824L45.726 22.1534L35.6129 32.2665L32.2419 28.8955Z" fill="#61DAFB"/>
</svg>"##;

    fs::write(path.join("fluxor.svg"), content)
        .expect("Failed to create assets/img/fluxor.svg for assets example");
}

