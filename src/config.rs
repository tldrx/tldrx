use std::path::{PathBuf, Path};
use std::fs;

use clap::Parser;
use anyhow::{anyhow, Result, Context};
use serde::Deserialize;

use crate::platform::Platform;


#[derive(Debug, Parser)]
#[clap(arg_required_else_help = true)]
#[clap(author, version, about)]
pub(crate) struct Args {
    /// Show pages for this command. <git | git merge | ...>
    #[clap(min_values = 1)]
    pub command: Vec<String>,

    /// Show pages for the given platform. Option: [osx | linux | ...]
    #[clap(short, long)]
    pub platform: Option<Platform>,

    /// Updates the offline cache of pages
    #[clap(short, long)]
    pub update: bool,

    /// Lists all pages for current platform or all with `-p all`.
    #[clap(short, long)]
    pub list: bool,

    /// Show pages for the given language. Option: [zh | zh_TW | ...]
    #[clap(short = 'L', long)]
    pub language: Option<String>,

    /// Style the output pages? Choice: [auto| on| off]
    #[clap(long)]
    pub styled: Option<StyledChoice>,

    /// Print version.
    #[clap(short = 'v', long)]
    pub version: bool,
}


#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
pub(crate) enum StyledChoice {
    Auto,
    On,
    Off,
}

impl std::str::FromStr for StyledChoice {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(StyledChoice::Auto),
            "on" => Ok(StyledChoice::On),
            "off" => Ok(StyledChoice::Off),
            _ => Err(anyhow!("Unknown styled choice: {}. Choice: [auto | on | off]", s))
        }
    }
}

impl Default for StyledChoice {
    fn default() -> Self {
        Self::Auto
    }
}


#[derive(Debug, Default)]
pub(crate) struct Config {
    pub official_pages_dir: Option<PathBuf>,
    pub private_pages_dir: Option<PathBuf>,
    pub platform: Option<Platform>,
    pub languages: Vec<String>,
    pub styled: StyledChoice,
}

impl Config {
    pub fn load_from(config_file: Option<&Path>) -> Result<Self> {
        match config_file {
            Some(f) if f.is_file() => {
                let content = fs::read_to_string(f).with_context(||
                    format!("Failed to read config file: {}", f.display())
                )?;
                let raw_config: RawConfig = toml::from_str(&content).with_context(||
                    format!("Failed to parse config file: {}", f.display())
                )?;
                let config: Config = raw_config.into();
                Ok(config)
            }
            _ => Ok(Default::default()),
        }
    }

    pub fn load() -> Result<Self> {
        let config_file = dirs::home_dir().map(|h| h.join(".tldrxrc"));
        Self::load_from(config_file.as_deref())
    }

    pub fn combine(&mut self, args: &Args) {
        if let Some(styled) = args.styled {
            self.styled = styled
        }
        if args.platform.is_some() {
            self.platform = args.platform.clone();
        }
        if let Some(lang) = &args.language {
            self.languages = vec![lang.to_string()];
        } else {
            // TODO: get lang from env
        }
    }
}

pub(crate) fn get_default_pages_dir() -> Result<PathBuf> {
    dirs::cache_dir().map(|d| d.join("tldr")).ok_or(anyhow!("Error getting default pages dir!"))
}

impl From<RawConfig> for Config {
    fn from(rc: RawConfig) -> Self {
        Self {
            official_pages_dir: rc.official_pages_dir,
            private_pages_dir: rc.private_pages_dir,
            platform: rc.platform.and_then(|p| p.parse().ok()),
            languages: Vec::new(),
            styled: rc.sytled.unwrap_or_default(),
        }
    }
}


#[derive(Debug, Deserialize)]
struct RawConfig {
    pub official_pages_dir: Option<PathBuf>,
    pub private_pages_dir: Option<PathBuf>,
    pub platform: Option<String>,
    pub sytled: Option<StyledChoice>,
}