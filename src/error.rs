use std::error::Error;
use std::path::Path;

const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const UNDERLINE: &str = "\x1b[4m";
const RESET: &str = "\x1b[0m";

pub fn print_error(error: impl std::fmt::Display) {
    eprintln!("{RED}error:{RESET} {error}");
    eprintln!();
    eprintln!("{UNDERLINE}Usage:{RESET} ptree [OPTIONS] [PATH]");
    eprintln!();
    eprintln!("For more information, try '--help'.");
}

pub fn validate_root(root: &Path) -> Result<(), Box<dyn Error>> {
    let metadata = match std::fs::metadata(root) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err(format!(
                "directory '{YELLOW}{}{RESET}' does not exist",
                root.display()
            )
            .into());
        }
        Err(error) => {
            return Err(
                format!("cannot access '{YELLOW}{}{RESET}': {error}", root.display()).into(),
            );
        }
    };

    if !metadata.is_dir() {
        return Err(format!("'{}' is not a directory", root.display()).into());
    }

    Ok(())
}
