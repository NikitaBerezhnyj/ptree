mod cli;
mod config;
mod renderer;
mod tree;

use std::error::Error;
use std::path::PathBuf;

use clap::Parser;

use crate::cli::Args;
use crate::config::{Config, Settings};
use crate::renderer::Renderer;
use crate::tree::build_tree;

fn main() {
    if let Err(error) = run(Args::parse()) {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let root = args.path.clone().unwrap_or_else(|| PathBuf::from("."));

    if args.init {
        let created = config::init(&root)?;
        println!("Created {}", created.display());
        return Ok(());
    }

    let config = if args.no_config {
        Config::default()
    } else if let Some(path) = &args.config {
        Config::load(path)?
    } else if let Some(path) = Config::discover(&root) {
        Config::load(&path)?
    } else {
        Config::default()
    };

    let settings = Settings::resolve(&args, config)?;
    let tree = build_tree(&root, &settings.options)?;

    print!(
        "{}",
        Renderer::render(&tree, settings.format, settings.stats)
    );

    Ok(())
}
