## v1.1.1

- Updated Fluxio from v0.5.0 to v0.5.1.
Note: To update the Fluxio crate, which is a major dependency module of the core Fluxor framework, we will update the PATCH version ("MAJOR.MINOR.PATCH") for each Fluxio dependency. This means updating one dependency crate at a time—such as moving from 0.5.0 to 0.5.1, then to 0.5.2, and so on—until all dependencies of the Fluxio crate have been updated.
By adopting this approach, if any new PATCH version of Fluxio introduces issues, it will be easier to revert to the previous version, thereby ensuring that the Fluxor framework remains stable and crash-free.
- Updated routes-project example. (+ dynamic route)
- Updated cans-template-engine example. (+ dynamic route)

## v1.1.0

- Crates updated:
    - cans (v1.3.0 => v1.5.0).
- Updated fluxor_template:
    - Added analytics page.
    - Added ds module.
    - Improved styles.

## v1.0.2

- Updated fluxor_template

## v1.0.0

- Added cans crate.
- Added Examples:
    - fluxor (fluxor-template).
- Updated Fluxor CLI Dependencies:
    - clap (v4.5.48 => v4.5.54).
    - regex (v1.11.3 => v1.12.2).
- Updated Fluxor CLI:
    - Added (lib) get_crate_version function.
    - Added (utils) copy_folder_dir function.
    - Added (metadata) create_license function.
    - Updated README.md.
    - Examples updated.

## v0.9.5

- Updated Fluxor Core Module:
    - Added set_custom_404 function.
    - Imported cans::content::* inside the prelude.
- Updated README.md.
- Examples updated.

## v0.9.4

- Crates removed:
    - serde.
    - serde_json.
- Modules  removed:
    - encode
- Examples updated.

## v0.9.3

- Crates updated:
    - serde (v1.0.228: features).
    - serde_json (v1.0.145: features).

## v0.9.2

- Renamed mysql module to mysql_async (mysql_async = "0.36.1")

## v0.9.1

- Crates updated:
    - cans (v1.1.0 -> v1.3.0).

## v0.9.0

- Crates updated:
    - wtime (v0.6.0 -> v0.7.0).
    - mathlab (v1.4.0 -> v1.5.0).
- Modules / Crates removed:
    - mongo / mongodb = "3.3.0" 
    - psql / tokio-postgres = "0.7.14"
    - redis / redis = "0.32.7"
- Modules / Crates added:
    - mysql / mysql_async = "0.36.1"

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