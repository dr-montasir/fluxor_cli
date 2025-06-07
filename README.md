<div align="center">
    <a href="https://github.com/dr-montasir/fluxor_cli">
        <img src="fluxor-icon-64x64.svg" width="100">
        <h2>FLUXOR CLI</h2>
    </a>
    <a href="https://github.com/dr-montasir/fluxor_cli" target="_blank">
        <img alt="github" src="https://img.shields.io/badge/github-dr%20montasir%20/%20fluxor_cli-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">
    </a>
    <a href="https://crates.io/crates/fluxor_cli" target="_blank">
        <img alt="crates.io" src="https://img.shields.io/crates/v/fluxor_cli.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">
    </a>
    <a href="https://docs.rs/fluxor_cli" target="_blank">
        <img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-fluxor_cli-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="22">
    </a>
    <a href="https://choosealicense.com/licenses/apache-2.0" target="_blank">
       <img alt="license" src="https://img.shields.io/badge/license-apache_2.0-4a98f7.svg?style=for-the-badge&labelColor=555555&logo=apache" height="22">
    </a>
    <a href="https://choosealicense.com/licenses/mit" target="_blank">
       <img alt="license" src="https://img.shields.io/badge/license-mit-4a98f7.svg?style=for-the-badge&labelColor=555555&logo=apache" height="22">
    </a>
    <a href="https://crates.io/crates/fluxor_cli" target="_blank">
        <img 
            alt="downloads" 
            src="https://img.shields.io/crates/d/fluxor_cli.svg?style=for-the-badge&labelColor=555555&logo=&color=428600"
            height="22"
        >
    </a>
    <a href="https://deps.rs/crate/fluxor_cli" target="_blank">
        <img 
            alt="Dependency Status" 
            src="https://deps.rs/crate/fluxor_cli/latest/status.svg?style=for-the-badge"
            height="22"
        >
    </a>
</div>

**Fluxor CLI** is the command-line interface for the Fluxor web framework, enabling rapid project scaffolding and management for Rust applications focused on data science and computing.

## Features

- **Project Scaffolding**: Quickly create new Fluxor projects with pre-configured templates.
- **Version Flexibility**: Specify the version of Fluxor to utilize in your projects.
- **Example Templates**: Easily initialize projects with example code to kickstart development.

## Getting Started

To install the Fluxor CLI, run:

```bash

cargo install fluxor_cli

```

## Usage

Once installed, you can create a new Fluxor project with:

```bash

fluxor new <project_name> --version latest --example helloworld

```
Replace <project_name> with your desired project name. Navigate to your project directory:

```bash

cd <project_name>

```

Running The Application
To build and run your newly created Fluxor application, use:

```bash

cargo run

```

The application should start on http://127.0.0.1:8080.

## All Examples

### Hello World
- helloworld
- helloworld-api
- helloworld-api-server

### Routes
- routes
- routes-project

### Assets
- assets

Use the example name after the flag --example (e.g., `helloworld`):

```terminal
fluxor new my_project --version latest --example helloworld

fluxor new my_app --version latest --example routes

fluxor new routes_app --version latest --example routes-project

fluxor new assets_example --version latest --example assets
```

## Documentation
For more detailed usage and advanced features, refer to the [fluxor](https://docs.rs/fluxor/latest/fluxor/) & [fluxor_cli](https://docs.rs/fluxor_cli/latest/fluxor_cli/) documentations.

## Contributing
Contributions are welcome! Fork the repository and submit a pull request. For larger changes, please discuss them in an issue.

## License
Fluxor CLI is licensed under either of the following licenses:

- MIT License
- Apache License, Version 2.0

See the LICENSE file for more details.

---

## Author

[Dr. Montasir Mirghani](https://github.com/dr-montasir)