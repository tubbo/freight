use merge::Merge;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::default::Default;
use std::fs::read_to_string;
use std::path::PathBuf;
use xdg::BaseDirectories;

/// Configuration for the Freight project.
///
/// This configuration comes from three different sources, in order of application:
///
/// 1. Default values, like the freight-lang source.
/// 2. Global configuration from `~/.config/freight/config.toml`.
/// 3. Project configuration from `./freight.toml`
/// 4. Environment variable overrides, using the `$FREIGHT_` prefix.
#[derive(Debug, Default, Deserialize, Merge, Serialize)]
pub struct Config {
    /// Configuration for all barges.
    #[merge(strategy = merge::vec::append)]
    barge: Vec<BargeConfig>,
}

/// Configure a Barge server to look for packages.
#[derive(Debug, Deserialize, Serialize)]
struct BargeConfig {
    /// URL to the Barge server.
    url: String,

    /// Optional username passed over HTTP Basic Auth.
    username: Option<String>,

    /// Optional token passed over HTTP Basic Auth.
    password: Option<String>,
}

impl PartialEq for BargeConfig {
    /// Barges are tested for equivalence by comparing their URLs.
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

/// Find global configuration from the XDG base directory in `~/.config/freight/config.toml`.
fn global() -> Result<Config, Error> {
    let xdg = BaseDirectories::with_prefix("freight")?;
    let path = xdg.place_config_file("config.toml")?;
    let source = read_to_string(path)?;

    toml::from_str(&source)
}

/// Find project configuration from the local root directory in `freight.toml`
fn local(path: &PathBuf) -> Result<Config, Error> {
    path.push("freight.toml");

    let source = read_to_string(path)?;

    toml::from_str(&source)
}

/// Find overrides in environment variables with the `$FREIGHT_` prefix.
fn env() -> Result<Config, Error> {
    envy::prefixed("FREIGHT_").from_env::<Config>()
}

/// Initialize the Freight configuration by merging together its sources.
pub fn init(root: &PathBuf) -> Result<Config, Error> {
    let config = Config::default();

    config.merge(global());
    config.merge(local(root));
    config.merge(env());

    Ok(config)
}
