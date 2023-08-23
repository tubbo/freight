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

/// Find project configuration from the local root directory in `freight.toml`
fn read(path: &PathBuf) -> Option<Config> {
    let source = read_to_string(path)?;
    let config: Config = config.parse()?;
    
    Ok(config)
}

/// Initialize the Freight configuration by merging together its sources.
pub fn init(root: &PathBuf) -> Result<Config, Error> {
    let config = Config::default();
    let xdg = BaseDirectories::with_prefix("freight").place_config_path("config.toml")?;
    
    if let Some(global) = read(xdg) {
        config.merge(global);
    }
    
    if let Some(local) = read(root.join("freight.toml")) {
        config.merge(local);
    }
    
    if let Ok(env) = envy::prefixed("FREIGHT_").from_env::<Config>() {
        config.merge(env);
    }

    Ok(config)
}
