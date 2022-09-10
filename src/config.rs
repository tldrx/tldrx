use std::path::{Path, PathBuf};
use std::{env, fs};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
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

    /// Edit command.
    #[clap(short, long)]
    pub edit: bool
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
    pub editor: Option<String>,
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
            self.languages = get_env_languages();
        }
    }

    pub fn get_official_page_dir(&self) -> Result<PathBuf> {
        match self.official_pages_dir {
            Some(ref d) => Ok(d.to_owned()),
            None => Ok(get_default_pages_dir()?),
        }
    }
}

fn get_env_languages() -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    let ignore = |lang: &String| {
        let lang = lang.to_uppercase();
        // vaild langnuage code at least with two char which also cover the 'C' scenario
        lang.len() >= 2 && lang != "POSIX"
    };

    let lang = env::var("LANG").ok().filter(ignore);
    match lang {
        None => return results,
        Some(lang) => {
            // an expension would gain a more accurate result
            let mut expension_push = |lang: &str| {
                results.push(lang.to_string());
                if lang.len() > 2 {
                    results.push(lang[0..2].to_string());
                }
            };

            if let Some(language) = env::var("LANGUAGE").ok().filter(ignore) {
                // LANGUAGE=l1:l2:...
                language.split(":").into_iter().for_each(|l| {
                    expension_push(l);
                });
            }
            // LANG=ll[_CC][.encoding]
            if let Some(l) = lang.split(".").into_iter().next() {
                expension_push(l);
            }
            return results;
        }
    };
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
            editor: rc.editor,
        }
    }
}


#[derive(Debug, Deserialize)]
struct RawConfig {
    pub official_pages_dir: Option<PathBuf>,
    pub private_pages_dir: Option<PathBuf>,
    pub platform: Option<String>,
    pub sytled: Option<StyledChoice>,
    pub editor: Option<String>,
}



#[cfg(test)]
mod test {

    mod language {
        use std::env;
        use std::sync::Mutex;

        use lazy_static::lazy_static;

        use crate::config::get_env_languages;

        const LANG: &str = "LANG";
        const LANGUAGE: &str = "LANGUAGE";

        lazy_static! {
            static ref MUTEX: Mutex<()> = Mutex::default();
        }

        fn clean_langs_env_run<F: FnOnce()>(f: F) {
            let _lock = MUTEX.lock();
            env::remove_var(LANG);
            env::remove_var(LANGUAGE);
            f();
        }

        #[test]
        fn missing_lang() {
            clean_langs_env_run(|| {
                env::set_var(LANGUAGE, "zh_TW:bo:en");
                assert_eq!(get_env_languages(), vec![String::new(); 0]);
            });
        }

        #[test]
        fn missing_language() {
            clean_langs_env_run(|| {
                env::set_var(LANG, "zh");
                assert_eq!(get_env_languages(), vec!["zh"]);
            });
        }

        #[test]
        fn lang_with_encoding() {
            clean_langs_env_run(|| {
                env::set_var(LANG, "zh.UTF-8");
                assert_eq!(get_env_languages(), vec!["zh"]);
            });
        }

        #[test]
        fn language_priority() {
            clean_langs_env_run(|| {
                env::set_var(LANG, "zh");
                env::set_var(LANGUAGE, "bo:en");
                assert_eq!(get_env_languages(), vec!["bo", "en", "zh"]);
            });
        }

        #[test]
        fn lang_and_language_expansion() {
            clean_langs_env_run(|| {
                env::set_var(LANG, "en_US");
                env::set_var(LANGUAGE, "zh_TW:bo");
                assert_eq!(
                    get_env_languages(),
                    vec!["zh_TW", "zh", "bo", "en_US", "en"]
                )
            });
        }

        #[test]
        fn ignore_c_and_posix() {
            clean_langs_env_run(|| {
                env::set_var(LANG, "C");
                assert_eq!(get_env_languages(), vec![String::new(); 0]);
                env::set_var(LANG, "POSIX");
                assert_eq!(get_env_languages(), vec![String::new(); 0]);
            });
        }
    }
}
