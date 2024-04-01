# {{project-name}}

Build and run with `cargo`, e.g. `cargo run -- -p 8080` to run a debug build on
port `8080`.

Use `RUST_LOG=debug` (or even `trace`) to get more detailed logging.

Use `#[tracing::instrument]` to instrument function calls for o11y and span traces.