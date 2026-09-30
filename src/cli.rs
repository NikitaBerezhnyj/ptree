use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "ptree")]
#[command(about = "Print project structure")]
pub struct Args {
    /// Root directory to scan
    pub path: Option<PathBuf>,

    /// Maximum directory depth
    #[arg(short, long)]
    pub depth: Option<usize>,

    /// Ignore .gitignore rules
    #[arg(long)]
    pub no_gitignore: bool,

    /// Additional ignore patterns
    #[arg(long, value_name = "PATTERN")]
    pub ignore: Vec<String>,

    /// Show directories only
    #[arg(long, conflicts_with = "files_only")]
    pub dirs_only: bool,

    /// Show files only
    #[arg(long, conflicts_with = "dirs_only")]
    pub files_only: bool,

    /// Filter files by extension
    #[arg(long, value_name = "EXT", value_delimiter = ',')]
    pub ext: Vec<String>,

    /// Show hidden files and directories
    #[arg(long)]
    pub hidden: bool,

    /// Show statistics
    #[arg(long)]
    pub stats: bool,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Tree)]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Tree,
    Compact,
    Json,
}
