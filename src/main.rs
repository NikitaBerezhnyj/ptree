mod cli;
mod renderer;
mod tree;

use std::path::PathBuf;

use clap::Parser;

use crate::cli::Args;
use crate::renderer::TreeRenderer;
use crate::tree::build_tree;

fn main() {
    let args = Args::parse();

    println!("path: {:?}", args.path);
    println!("depth: {:?}", args.depth);
    println!("no_gitignore: {}", args.no_gitignore);
    println!("ignore: {:?}", args.ignore);
    println!("dirs_only: {}", args.dirs_only);
    println!("files_only: {}", args.files_only);
    println!("ext: {:?}", args.ext);
    println!("hidden: {}", args.hidden);
    println!("stats: {}", args.stats);
    println!("format: {:?}", args.format);

    let root = args.path.unwrap_or_else(|| PathBuf::from("."));

    let tree = build_tree(&root, args.depth).unwrap_or_else(|error| {
        eprintln!("Error: {error}");
        std::process::exit(1);
    });

    let output = TreeRenderer::render(&tree);
    print!("{output}");
}
