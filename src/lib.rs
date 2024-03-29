use std::process;

use anyhow::{anyhow, Result};
use clap::Parser;

mod cache;
mod config;
mod page;
mod platform;


pub fn main() {
    let args = config::Args::parse();
    let mut config = match config::Config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{:?}", e);
            process::exit(1);
        }
    };
    config.combine(&args);

    match run(&args, &config) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{:?}", e);
            process::exit(1);
        }
    };
}

fn run(args: &config::Args, config: &config::Config) -> Result<()> {
    if args.update {
        return cache::update(config);
    }
    if args.list {
        return cache::list(config);
    }

    let command = &args.command.join("-");

    if args.edit {
        return cache::edit(&command, args, config);
    }

    let pages = cache::seek(&command, config)?;
    if pages.is_empty() {
        let bin = env!("CARGO_PKG_NAME");
        let msg = format!(
            "404: {}\n\n\
             Try:\n  \
               * {} -u\n  \
               * {} -e {}\n  \
               * https://github.com/tldr-pages/tldr/issues/new?title=page%20request:%20{}\
            ", command, bin, bin, command, command
        );
        return Err(anyhow!(msg));
    }
    for page in pages {
        page.render()?;
    }
    Ok(())
}
