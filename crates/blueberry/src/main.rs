use blueberry::web;
use blueberry_config::Config;
use clap::Parser;

mod cli;

/// Runs the axum server, exiting with an error message on failure.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The cli flag wins over `BLUEBERRY_PROFILE` and the default profile.
    let cli = cli::Cli::parse();
    let config = match cli.profile {
        Some(profile) => Config::load_for(&profile)?,
        None => Config::load()?,
    };
    let addr = (config.server.host.as_str(), config.server.port);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!(
        "Blueberry listening on {}:{}",
        config.server.host, config.server.port
    );

    axum::serve(listener, web()).await?;
    Ok(())
}
