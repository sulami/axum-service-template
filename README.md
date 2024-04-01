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

Build and run with `cargo`, e.g.
```sh
`cargo run -- --port 8080 --database-url "postgres://$(whoami)@localhost:5432/postgres"
```
to run a debug build on port `8080`.

Use `RUST_LOG=trace` to get more detailed logging. The default level is `info`.

Use `#[tracing::instrument]` to instrument function calls for o11y and span traces.