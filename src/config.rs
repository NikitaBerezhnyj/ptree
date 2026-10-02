use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::cli::{Args, OutputFormat};
use crate::tree::WalkOptions;

pub const CONFIG_FILE_NAME: &str = ".ptreerc";

const DEFAULT_CONFIG: &str = r#"# ptree configuration
# CLI flags always take priority over values from this file.

# Maximum directory depth (omit for unlimited)
# depth = 3

hidden = false
gitignore = true
stats = false
dirs_only = false
files_only = false

# tree | compact | json
format = "tree"

# Show only these extensions (CLI --ext replaces this list)
ext = []

# Extra ignore patterns (CLI --ignore is added to this list)
ignore = []
"#;

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub depth: Option<usize>,
    pub hidden: Option<bool>,
    pub gitignore: Option<bool>,
    pub stats: Option<bool>,
    pub dirs_only: Option<bool>,
    pub files_only: Option<bool>,
    pub format: Option<OutputFormat>,
    pub ext: Vec<String>,
    pub ignore: Vec<String>,
}

#[derive(Debug)]
pub enum ConfigError {
    Read {
        path: PathBuf,
        source: io::Error,
    },
    Parse {
        path: PathBuf,
        source: toml::de::Error,
    },
    DirsAndFilesOnly,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => {
                write!(f, "cannot read {}: {source}", path.display())
            }
            Self::Parse { path, source } => {
                write!(f, "invalid config {}:\n{source}", path.display())
            }
            Self::DirsAndFilesOnly => {
                write!(f, "`dirs_only` and `files_only` cannot both be enabled")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path).map_err(|source| ConfigError::Read {
            path: path.to_path_buf(),
            source,
        })?;

        toml::from_str(&content).map_err(|source| ConfigError::Parse {
            path: path.to_path_buf(),
            source,
        })
    }

    pub fn discover(start: &Path) -> Option<PathBuf> {
        let start = fs::canonicalize(start).ok()?;

        start
            .ancestors()
            .map(|dir| dir.join(CONFIG_FILE_NAME))
            .find(|candidate| candidate.is_file())
    }
}

pub fn init(dir: &Path) -> io::Result<PathBuf> {
    let path = dir.join(CONFIG_FILE_NAME);

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|e| io::Error::new(e.kind(), format!("{}: {e}", path.display())))?;

    file.write_all(DEFAULT_CONFIG.as_bytes())?;

    Ok(path)
}

pub struct Settings {
    pub options: WalkOptions,
    pub format: OutputFormat,
    pub stats: bool,
}

impl Settings {
    pub fn resolve(args: &Args, config: Config) -> Result<Self, ConfigError> {
        let (dirs_only, files_only) = if args.dirs_only || args.files_only {
            (args.dirs_only, args.files_only)
        } else {
            (
                config.dirs_only.unwrap_or(false),
                config.files_only.unwrap_or(false),
            )
        };

        if dirs_only && files_only {
            return Err(ConfigError::DirsAndFilesOnly);
        }

        let extensions: Vec<String> = if args.ext.is_empty() {
            config.ext
        } else {
            args.ext.clone()
        }
        .into_iter()
        .map(|e| e.trim_start_matches('.').to_owned())
        .collect();

        let mut ignore = config.ignore;
        ignore.extend(args.ignore.iter().cloned());

        let options = WalkOptions {
            max_depth: args.depth.or(config.depth),
            show_hidden: flag(args.hidden, args.no_hidden, config.hidden, false),
            dirs_only,
            files_only,
            extensions,
            ignore,
            use_gitignore: flag(args.gitignore, args.no_gitignore, config.gitignore, true),
        };

        Ok(Self {
            options,
            format: args.format.or(config.format).unwrap_or_default(),
            stats: flag(args.stats, args.no_stats, config.stats, false),
        })
    }
}

fn flag(on: bool, off: bool, from_config: Option<bool>, default: bool) -> bool {
    if on {
        true
    } else if off {
        false
    } else {
        from_config.unwrap_or(default)
    }
}
