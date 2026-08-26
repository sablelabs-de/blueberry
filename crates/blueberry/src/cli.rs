use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "blueberry", about = "Blueberry server CLI", version)]
pub struct Cli {
    /// Config profile to load: `blueberry.<profile>.toml`.
    ///
    /// Wins over the `BLUEBERRY_PROFILE` environment variable.
    /// Defaults to 'development'.
    #[arg(short = 'p', long = "profile", value_name = "PROFILE")]
    pub profile: Option<String>,
}
