# Axum Service Template

## Usage

1. Ensure you have `cargo-generate`:
   ```sh
   cargo install cargo-generate
   ```
   (`cargo binstall` also works.)
2. Setup a new project using
   ```sh
   cargo generate --git https://github.com/sulami/axum-service-template
   ```
   and follow the prompts.
3. Remove any content above this line.
---

# {{project-name}}

Build and run with `cargo`, e.g. `cargo run -- -p 8080` to run a debug build on
port `8080`.

Use `RUST_LOG=debug` (or even `trace`) to get more detailed logging.

Use `#[tracing::instrument]` to instrument function calls for o11y and span traces.