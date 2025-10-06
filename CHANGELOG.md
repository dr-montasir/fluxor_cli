## v0.8.0

- Functions changed:
    - Started from version 0.8.0:
        - The order of parameters in the route function has been updated from (path, method, handler) to (method, path, handler).
        - This change improves code readability and consistency by placing the HTTP method first, aligning with common conventions and making it easier to group related methods together for clarity.
- Crates updated:
    - cargo_metadata (v0.20.0 -> v0.23.0).
    - clap (v4.5.40 -> v4.5.48).
    - regex (v1.11.1 -> v1.11.3).
- Added the structure of the db project example.

## v0.7.1

- Added: cans-template-engine example.

## v0.7.0

- Added the metadata module.
- Modified all example.
- Added a dotenv example.

## v0.6.1

- Modified assets example.
- Updated clap version (v4.5.39 -> v4.5.40).

## v0.6.0

- Added assets example.

## v0.5.0

- Added routes-project example.
- Updated clap version (v4.5.38 -> v4.5.39).
- Updated cargo_metadata version (v0.19.2 -> v0.20.0).

## v0.4.0

- Added routes example.

## v0.3.0

- Used command **fluxor** instead of **fluxor_cli**.
- Updated clap version (v4.5.32 -> v4.5.38).

## v0.2.0

- Moved Fluxor-cli to its own crate.

## 0.1.0

- Initial version.