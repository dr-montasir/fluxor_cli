#![doc(html_logo_url = "https://github.com/dr-montasir/fluxor_cli/raw/HEAD/fluxor-icon-64x64.svg")]
#![doc = r"<div align='center'><a href='https://github.com/dr-montasir/fluxor_cli' target='_blank'><img src='https://github.com/dr-montasir/fluxor_cli/raw/HEAD/fluxor-icon-64x64.svg' alt='Fluxor CLI' width='80' height='auto' /></a><br><br><a href='https://github.com/dr-montasir/fluxor_cli' target='_blank'>FLUXOR</a><br><br>Fluxor_cli is the command-line interface for the Fluxor web framework, enabling rapid project scaffolding and management for Rust applications focused on data science and computing.</div>"]

pub mod utils;
pub mod metadata;
mod examples;

pub use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::process::Command as ProcessCommand;

use utils::*;
use examples::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    New {
        name: String,
        #[clap(short, long, default_value = "latest")]
        version: String,
        #[clap(short, long, default_value = "helloworld")]
        example: String,
    },
}

pub fn fetch_latest_version(crate_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = ProcessCommand::new("cargo")
        .arg("search")
        .arg(crate_name)
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Failed to fetch latest version of {}: {}",
            crate_name,
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("Cargo search output: {}", stdout); // Debug print

    // Use a regex to extract the version number
    let re = Regex::new(r#"(\d+\.\d+\.\d+)"#).unwrap();
    let version = stdout
        .lines()
        .find(|line| line.contains(crate_name))
        .and_then(|line| re.find(line))
        .map(|version| version.as_str())
        .ok_or_else(|| format!("Crate '{}' not found", crate_name))?;

    println!("Parsed version: {}", version); // Debug print

    Ok(version.to_string())
}

// Fetch dependencies crate version if "latest" is specified
pub fn get_crate_version(version: &str, dep_crate_name: &str) -> String {
    if version == "latest" {
        fetch_latest_version(dep_crate_name).expect(&format!("Failed to fetch latest version of {}", dep_crate_name))
    } else {
        version.to_string()
    }
}

pub fn create_fluxor_web_project(name: &str, version: &str, example: &str) {
    let package_name = to_crate_name(name);
    let project_path = Path::new(&package_name);

    // Check if project directory already exists
    if project_path.exists() {
        eprintln!("Project directory already exists.");
        return;
    }

    // Create project directory
    fs::create_dir_all(&project_path).expect("Failed to create project directory");

    // Fetch fluxor version if "latest" is specified
    let fluxor_version = if version == "latest" {
        fetch_latest_version("fluxor").expect("Failed to fetch latest version of fluxor")
    } else {
        version.to_string()
    };

    // create README.md
    metadata::create_readme(project_path, &package_name);

    // Create Cargo.toml specific for the example
    let cargo_toml = match example {
        // Hello World Examples
        "helloworld" => hello_world::hello_world_cargo_toml(&package_name, &fluxor_version),
        "helloworld-api" => hello_world::hello_world_cargo_toml(&package_name, &fluxor_version),
        "helloworld-api-server" => hello_world::hello_world_api_server_cargo_toml(&package_name, &fluxor_version),
        // Routes Examples
        "routes" => routes::routes_cargo_toml(&package_name, &fluxor_version),
        "routes-project" => routes::routes_cargo_toml(&package_name, &fluxor_version),
        // Assets Examples
        "assets" => assets::assets_cargo_toml(&package_name, &fluxor_version),
        // DotEnv Examples
        "dotenv" => dotenv::dotenv_cargo_toml(&package_name, &fluxor_version),
        "cans-template-engine" => cans::template_cargo_toml(&package_name, &fluxor_version),
        // DB
        "db-redis" => db::db_cargo_toml(&package_name, &fluxor_version),
        // Fluxor Full Template Example
        "fluxor-template" => fluxor::fluxor_template_cargo_toml(
            &package_name, 
            &get_crate_version("latest", "crator"), 
            &get_crate_version("latest", "fluxor")
        ),
        _ => {
            eprintln!("Unknown example specified: {}", example);
            return;
        }
    };

    fs::write(project_path.join("Cargo.toml"), cargo_toml)
        .expect("Failed to create Cargo.toml");

    // Create src directory under the project
    let src_path = project_path.join("src");
    fs::create_dir_all(&src_path).expect("Failed to create src directory");

    // Create src folders and files based on the specified example
    match example {
        // Hello World Examples
        "helloworld" => {
            // metadata files
            hello_world::config_metadata(&project_path);

            // src/main.rs
            hello_world::hello_world_main_rs(&src_path);
        }
        "helloworld-api" => {
            // metadata files
            hello_world::config_metadata(&project_path);

            // src/main.rs
            hello_world::hello_world_api_main_rs(&src_path);
        }
        "helloworld-api-server" => {
            // metadata files
            hello_world::config_metadata(&project_path);

            // src/server.rs
            hello_world::hello_world_api_server_rs(&src_path);
        }
        // Routes Examples
        "routes" => {
            // metadata files
            routes::config_metadata(&project_path);

            // src/main.rs
            routes::routes_main_rs(&src_path);
        }
        "routes-project" => {
            // metadata files
            routes::config_metadata(&project_path);

            // src
            // src/main.rs
            routes::routes_project_main_rs(&package_name, &src_path);
            // src/lib.rs
            routes::routes_project_lib_rs(&src_path);

            // src/routes
            // Create routes directory under the src folder
            let src_routes_path = project_path.join("src/routes");
            fs::create_dir_all(&src_routes_path).expect("Failed to create src/routes directory");
            // src/routes/mod.rs
            routes::routes_project_routes_mod_rs(&src_routes_path);
            
            // src/routes/api
            // Create routes/api directory under the src folder
            let src_routes_api_path = project_path.join("src/routes/api");
            fs::create_dir_all(&src_routes_api_path).expect("Failed to create src/routes/api directory");
            // src/routes/api/mod.rs
            routes::routes_project_routes_api_mod_rs(&src_routes_api_path);
            // src/routes/api/msg.rs
            routes::routes_project_routes_api_msg_rs(&src_routes_api_path);

            // src/routes/pages
            // Create routes/pages directory under the src folder
            let src_routes_pages_path = project_path.join("src/routes/pages");
            fs::create_dir_all(&src_routes_pages_path).expect("Failed to create src/routes/pages directory");
            // src/routes/pages/mod.rs
            routes::routes_project_routes_pages_mod_rs(&src_routes_pages_path);
            // src/routes/pages/home.rs
            routes::routes_project_routes_pages_home_rs(&src_routes_pages_path);
            // src/routes/pages/about.rs
            routes::routes_project_routes_pages_about_rs(&src_routes_pages_path);
        }
        // Assets Examples
        "assets" => {
            // metadata files
            assets::config_metadata(&project_path);

            // main.rs
            assets::assets_main_rs(&src_path);

            // assets
            let assets_img_path = project_path.join("assets/img");
            let assets_css_path = project_path.join("assets/css");
            let assets_js_path = project_path.join("assets/js");
            
            fs::create_dir_all(&assets_img_path).expect("Failed to create sassets/img directory");
            fs::create_dir_all(&assets_css_path).expect("Failed to create sassets/css directory");
            fs::create_dir_all(&assets_js_path).expect("Failed to create sassets/js directory");
            
            assets::assets_img_fluxor_svg(&assets_img_path);
            assets::assets_css_styels_css(&assets_css_path);
            assets::assets_js_script_js(&assets_js_path);
        }
        // DotEnv Examples
        "dotenv" => {
            // metadata files
            dotenv::config_metadata(&project_path);

            // main.rs
            dotenv::dotenv_main_rs(&src_path);
        }
        // Cans:
        // Template Examples
        "cans-template-engine" => {
            // metadata files
            cans::config_metadata(&project_path);

            // main.rs
            cans::template_main_rs(&src_path);
        }
        // DB
        // REDIS DB
        "db-redis" => {
            // metadata files
            db::config_redis_metadata(&project_path);

            // main.rs
            db::db_redis_main_rs(&src_path);
        }
        // Fluxor
        // Full Fluxor Template Example
        "fluxor-template" => {
            // paths
            let assets_path = project_path.join("src/assets");
            let assets_images_path = project_path.join("src/assets/images");
            let assets_css_path = project_path.join("src/assets/css");
            let assets_js_path = project_path.join("src/assets/js");
            let components_path = project_path.join("src/components");
            let components_scripts_path = project_path.join("src/components/scripts");
            let db_path = project_path.join("src/db");
            let helpers_path = project_path.join("src/helpers");
            let routes_path = project_path.join("src/routes");
            let routes_api_path = project_path.join("src/routes/api");
            let routes_api_greeting_path = project_path.join("src/routes/api/greeting");
            let routes_pages_path = project_path.join("src/routes/pages");
            let validators_path = project_path.join("src/validators");

            // metadata files
            fluxor::config_metadata(&project_path);

            // src

            // src/main.rs
            fluxor::fluxor_template_main_rs(&package_name, &src_path);
            // src/lib.rs
            fluxor::fluxor_template_lib_rs(&src_path);

            // src/assets

            // src/assets/images
            fs::create_dir_all(assets_path.join("images")).expect("Failed to create assets/images directory");
            // src/assets/css
            fs::create_dir_all(assets_path.join("css")).expect("Failed to create assets/css directory");
            // src/assets/js
            fs::create_dir_all(assets_path.join("js")).expect("Failed to create assets/js directory");

            // src/assets/manifest.json
            fluxor::assets_manifest_json(&assets_path);
            // src/assets/service-worker.js
            fluxor::assets_service_worker_js(&assets_path);
            // src/assets/images/logo.svg
            fluxor::assets_images_logo_svg(&assets_images_path);
            // src/assets/css/styles.css
            fluxor::assets_css_styles_css(&assets_css_path);
            // src/assets/js/alpine.min.js
            fluxor::assets_js_alpine_min_js(&assets_js_path);

            // src/components

            // src/components/scripts
            fs::create_dir_all(components_path.join("scripts")).expect("Failed to create components/scripts directory");

            // src/components/scripts/mod.rs
            fluxor::components_scripts_mod_rs(&components_scripts_path);
            // src/components/scripts/sw_register.rs
            fluxor::components_scripts_sw_register_rs(&components_scripts_path);
            // src/components/badge.rs
            fluxor::components_badge_rs(&components_path);
            // src/components/footer.rs
            fluxor::components_footer_rs(&components_path);
            // src/components/head.rs
            fluxor::components_head_rs(&components_path);
            // src/components/header.rs
            fluxor::components_header_rs(&components_path);
            // src/components/layout.rs
            fluxor::components_layout_rs(&components_path);
            // src/components/logo.rs
            fluxor::components_logo_rs(&components_path);
            // src/components/mod.rs
            fluxor::components_mod_rs(&components_path);
            // src/components/nav.rs
            fluxor::components_nav_rs(&components_path);
            // src/components/notfound.rs
            fluxor::components_notfound_rs(&components_path);

            // src/db

            fs::create_dir_all(&db_path).expect("Failed to create src/db directory");

            // src/db/mod.rs
            fluxor::db_mod_rs(&db_path);

            // src/helpers

            fs::create_dir_all(&helpers_path).expect("Failed to create src/helpers directory");

            // src/helpers/mod.rs
            fluxor::helpers_mod_rs(&helpers_path);

            // src/routes

            fs::create_dir_all(&routes_path).expect("Failed to create src/routes directory");
            
            // src/routes/pages
            fs::create_dir_all(routes_path.join("pages")).expect("Failed to create routes/pages directory");
            // src/routes/api
            fs::create_dir_all(routes_path.join("api")).expect("Failed to create routes/api directory");
            // src/routes/api/greeting
            fs::create_dir_all(routes_path.join("api/greeting")).expect("Failed to create routes/api/greeting directory");

            // src/routes/mod.rs
            fluxor::routes_mod_rs(&routes_path);
            // src/routes/api/mod.rs
            fluxor::routes_api_mod_rs(&routes_api_path);
            // src/routes/api/greeting/mod.rs
            fluxor::routes_api_greeting_mod_rs(&routes_api_greeting_path);
            // src/routes/pages/mod.rs
            fluxor::routes_pages_mod_rs(&routes_pages_path);
            // src/routes/pages/home.rs
            fluxor::routes_pages_home_rs(&routes_pages_path);
            
            // src/validators

            fs::create_dir_all(&validators_path).expect("Failed to create src/validators directory");

            // src/validators/mod.rs
            fluxor::validators_mod_rs(&validators_path);
        }
        _ => {
            eprintln!("Unknown example specified: {}", example);
            return;
        }
    }

    println!("Fluxor project '{}' created successfully using the '{}' example.", package_name, example);
}
