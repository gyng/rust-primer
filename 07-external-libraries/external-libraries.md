# External libraries

- Crates are your packages
- crates.io is the NPM/PyPI equivalent
- Can add dependencies install from git, local, workspace (monorepo)
- To add a dependency, add it to `Cargo.toml` using `cargo add serde`
- Private/mirrored registries: https://doc.rust-lang.org/cargo/reference/registries.html
- `Cargo.lock` > `package-lock.json`, `Gemfile.lock`

## Go-to community libraries

For an up-to-date list: https://blessed.rs/crates

| Problem            | Crate                              |
| ------------------ | ---------------------------------- |
| Format parsing     | **serde**                          |
| CLI parsing        | **clap**                           |
| Error handling     | anyhow, thiserror                  |
| Network request    | reqwest, hyper                     |
| Web framework      | axum, actix-web, surf, warp, tower |
| ORM, query builder | sea-orm, diesel                    |
| Async runtime      | tokio                              |

## Objectives

- Import external libraries
- Add external libraries

## Exercise

Guided example on adding external libraries.

We will be using two libraries to create our profile printer.

- `serde` and `serde_json` to deserialise JSON into structs
- `clap` to parse CLI arguments.

To begin,

- Add `serde`, `clap` as libraries to `Cargo.toml` (you just have to uncomment)
- import the traits provided by the libraries (`clap::Clap`, `serde::{Deserialize, Serialize}`)
- Annotate structs with `Serialize`, `Deserialize`
- Fill out `main`
