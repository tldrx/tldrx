use std::path::PathBuf;

use anyhow::Result;

use crate::config::{self, Config};
use crate::page::{Kind, Page};
use crate::platform::Platform;

pub(crate) fn seek<'a>(command: &'a str, config: &'a Config) -> Result<Vec<Page<'a>>> {
    let pages_dir = match config.official_pages_dir {
        Some(ref d) => d.to_owned(),
        None => config::get_default_pages_dir()?,
    };

    let mut lang_folders = Vec::with_capacity(config.languages.len() + 1);
    for lang in &config.languages {
        if lang == "en" {
            break;
        }
        lang_folders.push(format!("pages.{}", lang));
    }
    lang_folders.push("pages".to_string());

    let platform = config.platform.as_ref().unwrap_or(&Platform::Common);

    let filename = &format!("{}.md", command);

    let mut pages = Vec::with_capacity(2);
    if let Some(page) = do_seek(
        pages_dir,
        &lang_folders,
        platform,
        filename,
        Kind::Official,
        config,
    ) {
        pages.push(page)
    }

    if let Some(ref d) = config.private_pages_dir {
        if let Some(page) = do_seek(
            d.to_owned(),
            &lang_folders,
            platform,
            filename,
            Kind::Private,
            config,
        ) {
            pages.push(page)
        }
    }

    Ok(pages)
}

fn do_seek<'a, T>(
    pages_dir: PathBuf,
    lang_folders: &[T],
    platform: &Platform,
    filename: &str,
    kind: Kind,
    config: &'a Config,
) -> Option<Page<'a>>
where
    T: AsRef<str> + std::convert::AsRef<std::path::Path>,
{
    let lang_dirs = lang_folders.iter().map(|f| pages_dir.join(f));
    for dir in lang_dirs {
        let file = dir.join(platform.to_string()).join(filename);
        if let Some(page) = Page::option_from(file, kind, platform.clone(), config) {
            return Some(page);
        }
        if *platform != Platform::Common {
            let file = dir.join(Platform::Common.to_string()).join(filename);
            if let Some(page) = Page::option_from(file, kind, Platform::Common, config) {
                return Some(page);
            }
        }
    }
    None
}
