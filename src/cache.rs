use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::config::{self, Config};
use crate::page::{Kind, Page};
use crate::platform::Platform;


const OFFICIAL_PAGES_ARCHIVE_URL: &str = "https://tldr.sh/assets/tldr.zip";
const PAGES_DIR: &str = "tldr-pages";


pub(crate) fn seek<'a>(command: &'a str, config: &'a Config) -> Result<Vec<Page<'a>>> {
    let pages_dir = match config.official_pages_dir {
        Some(ref d) => d.to_owned(),
        None => config::get_default_pages_dir()?,
    }.join(PAGES_DIR);

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

pub(crate) fn update(config: &Config) -> Result<()> {
    let dir = config.get_official_page_dir()?;
    fs::create_dir_all(&dir)
        .with_context(|| format!("Fail to create directory: {}", &dir.display()))?;

    let filename = download_archive(config)?;
    let file = File::open(&filename)
        .with_context(|| format!("Could not open file: {}", &filename.display()))?;
    let mut archive = zip::ZipArchive::new(BufReader::new(file))
        .with_context(|| format!("Error preparing to unzip: {}", &filename.display()))?;

    let tmp_dir = dir.join("tmp-pages");
    let pages_dir = dir.join(PAGES_DIR);

    archive
        .extract(&tmp_dir)
        .with_context(|| format!("Fail to extract archive to: {}", &tmp_dir.display()))?;
    if pages_dir.is_dir() {
        fs::remove_dir_all(&pages_dir)
            .with_context(|| format!("Fail to clean up: {}", &pages_dir.display()))?;
    }
    fs::rename(&tmp_dir, &pages_dir).with_context(|| {
        format!(
            "Error swapping dir: {} -> {}",
            &tmp_dir.display(),
            &pages_dir.display()
        )
    })?;
    fs::remove_file(&filename)
        .with_context(|| format!("Fail to clean up archive: {}", &filename.display()))?;
    Ok(())
}

fn download_archive(config: &Config) -> Result<PathBuf> {
    let url = OFFICIAL_PAGES_ARCHIVE_URL;
    let mut resp = reqwest::blocking::get(url)?
        .error_for_status()
        .with_context(|| format!("Fail to download archive form: {}", url))?;
    let dir = config.get_official_page_dir()?;

    let archive = dir.join("tldr.zip");

    let mut file = File::create(&archive)
        .with_context(|| format!("Fail to create archive: {}", archive.display()))?;
    let mut buf = BufWriter::new(&mut file);
    resp.copy_to(&mut buf).with_context(|| format!("Fail to copy archive stream"))?;
    Ok(archive)
}
