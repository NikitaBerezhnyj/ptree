use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use serde::Deserialize;

#[derive(Debug, Parser)]
#[command(
    name = "ptree",
    version,
    about = "Print project structure",
    long_about = "A CLI tool for displaying project structure in a tree-like format."
)]
pub struct Args {
    /// Root directory to scan
    pub path: Option<PathBuf>,

    /// Limit the depth of the directory tree
    #[arg(short = 'd', long, value_name = "N")]
    pub depth: Option<usize>,

    /// Disable .gitignore rules
    #[arg(long, conflicts_with = "gitignore")]
    pub no_gitignore: bool,

    /// Enable .gitignore rules
    #[arg(long, conflicts_with = "no_gitignore")]
    pub gitignore: bool,

    /// Ignore files and directories matching the given patterns
    ///
    /// Accepts several values: `--ignore "*.dart" "*.yaml"` or `--ignore "*.dart,*.yaml"`.
    #[arg(short = 'i', long, value_name = "PATTERN", num_args = 1.., value_delimiter = ',')]
    pub ignore: Vec<String>,

    /// Show directories only
    #[arg(short = 'D', long, conflicts_with = "files_only")]
    pub dirs_only: bool,

    /// Show files only
    #[arg(short = 'f', long, conflicts_with = "dirs_only")]
    pub files_only: bool,

    /// Filter files by extension
    ///
    /// Accepts several values: `--ext dart yaml` or `--ext dart,yaml`.
    #[arg(short = 'e', long, value_name = "EXT", num_args = 1.., value_delimiter = ',')]
    pub ext: Vec<String>,

    /// Show hidden files and directories
    #[arg(short = 'H', long, conflicts_with = "no_hidden")]
    pub hidden: bool,

    /// Hide hidden files and directories
    #[arg(long)]
    pub no_hidden: bool,

    /// Show statistics about the scanned project
    #[arg(short = 's', long, conflicts_with = "no_stats")]
    pub stats: bool,

    /// Hide statistics about the scanned project
    #[arg(long)]
    pub no_stats: bool,

    /// Set the output format
    #[arg(long, value_enum, value_name = "FORMAT")]
    pub format: Option<OutputFormat>,

    /// Create a default .ptreerc configuration file
    #[arg(long)]
    pub init: bool,

    /// Load configuration from a specific file
    #[arg(long, value_name = "FILE", conflicts_with = "no_config")]
    pub config: Option<PathBuf>,

    /// Disable configuration file loading
    #[arg(long)]
    pub no_config: bool,
}

#[derive(Debug, Clone, Copy, Default, ValueEnum, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    /// Tree-like hierarchical output
    #[default]
    Tree,

    /// Compact one-line-per-entry output
    Compact,

    /// JSON output
    Json,
}
