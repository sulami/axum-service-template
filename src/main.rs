//! Main module

use clap::Parser;
use color_eyre::Result;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;

mod server;

/// Main entry point
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(ErrorLayer::default())
        .init();
    color_eyre::install()?;

    let cli = Cli::parse();
    server::run(cli.port).await?;

    Ok(())
}

/// The command line arguments parsed by `clap`
#[derive(Parser)]
struct Cli {
    /// The port to listen on
    #[clap(short, long, env = "{{project-name | shouty_snake_case}}_PORT")]
    port: u16,
}
