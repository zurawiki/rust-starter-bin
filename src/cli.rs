use clap::{command, Parser};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser, Debug, Clone, Eq, PartialEq)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CliArgs {}

pub(crate) async fn main() -> anyhow::Result<()> {
    let _cli = CliArgs::parse();

    // Set RUST_LOG if not set
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "warn");
    }

    // Setup tracing subscriber so that library can log the rate limited message
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    println!("Hello, World!");
    Ok(())
}

#[test]
fn test_cli() {
    let cli = CliArgs::parse();
    assert_eq!(cli, CliArgs {});
}
