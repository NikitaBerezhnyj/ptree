mod cli;
mod renderer;
mod tree;

use std::path::PathBuf;

use clap::Parser;

use crate::cli::Args;
use crate::renderer::TreeRenderer;
use crate::tree::{WalkOptions, build_tree};

fn main() {
    let args = Args::parse();

    let root = args.path.unwrap_or_else(|| PathBuf::from("."));

    let options = WalkOptions {
        max_depth: args.depth,
        show_hidden: args.hidden,
        dirs_only: args.dirs_only,
        files_only: args.files_only,
        extensions: args.ext,
        ignore: args.ignore,
        use_gitignore: !args.no_gitignore,
    };

    let tree = build_tree(&root, &options).unwrap_or_else(|error| {
        eprintln!("Error: {error}");
        std::process::exit(1);
    });

    let output = if args.stats {
        TreeRenderer::render_stats(&tree)
    } else {
        TreeRenderer::render(&tree)
    };
    print!("{}", output);
}
