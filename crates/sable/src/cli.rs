use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "sable", about = "Sable server CLI", version)]
pub struct Cli {
  /// Config profile to load: `sable.<profile>.toml`.
  ///
  /// Wins over the `SABLE_PROFILE` environment variable.
  /// Defaults to 'development'.
  #[arg(short = 'p', long = "profile", value_name = "PROFILE")]
  pub profile: Option<String>,
}
