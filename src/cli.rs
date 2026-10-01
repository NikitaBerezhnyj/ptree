use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use serde::Deserialize;

#[derive(Debug, Parser)]
#[command(name = "ptree")]
#[command(about = "Print project structure")]
pub struct Args {
    pub path: Option<PathBuf>,

    #[arg(short, long)]
    pub depth: Option<usize>,

    #[arg(long, conflicts_with = "gitignore")]
    pub no_gitignore: bool,

    #[arg(long)]
    pub gitignore: bool,

    #[arg(long, value_name = "PATTERN")]
    pub ignore: Vec<String>,

    #[arg(long, conflicts_with = "files_only")]
    pub dirs_only: bool,

    #[arg(long, conflicts_with = "dirs_only")]
    pub files_only: bool,

    #[arg(long, value_name = "EXT", value_delimiter = ',')]
    pub ext: Vec<String>,

    #[arg(long, conflicts_with = "no_hidden")]
    pub hidden: bool,

    #[arg(long)]
    pub no_hidden: bool,

    #[arg(long, conflicts_with = "no_stats")]
    pub stats: bool,

    #[arg(long)]
    pub no_stats: bool,

    #[arg(long, value_enum)]
    pub format: Option<OutputFormat>,

    #[arg(long)]
    pub init: bool,

    #[arg(long, value_name = "FILE", conflicts_with = "no_config")]
    pub config: Option<PathBuf>,

    #[arg(long)]
    pub no_config: bool,
}

#[derive(Debug, Clone, Copy, Default, ValueEnum, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    #[default]
    Tree,
    Compact,
    Json,
}
