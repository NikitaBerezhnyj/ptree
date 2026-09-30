use std::path::Path;

use ignore::gitignore::{Gitignore, GitignoreBuilder};

pub struct IgnoreMatcher {
    custom_patterns: Vec<String>,
    gitignore: Option<Gitignore>,
}

impl IgnoreMatcher {
    pub fn new(root: &Path, patterns: &[String], use_gitignore: bool) -> Self {
        let gitignore = if use_gitignore {
            load_gitignore(root)
        } else {
            None
        };

        Self {
            custom_patterns: patterns.to_vec(),
            gitignore,
        }
    }

    pub fn is_ignored(&self, path: &Path) -> bool {
        if self.matches_custom_pattern(path) {
            return true;
        }

        self.gitignore
            .as_ref()
            .is_some_and(|gitignore| gitignore.matched(path, path.is_dir()).is_ignore())
    }

    fn matches_custom_pattern(&self, path: &Path) -> bool {
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            return false;
        };

        self.custom_patterns
            .iter()
            .any(|pattern| matches_pattern(name, pattern))
    }
}

fn load_gitignore(root: &Path) -> Option<Gitignore> {
    let path = root.join(".gitignore");

    if !path.is_file() {
        return None;
    }

    let mut builder = GitignoreBuilder::new(root);

    if builder.add(&path).is_some() {
        return None;
    }

    builder.build().ok()
}

fn matches_pattern(name: &str, pattern: &str) -> bool {
    if pattern == name {
        return true;
    }

    if let Some(extension) = pattern.strip_prefix("*.") {
        return name.ends_with(&format!(".{extension}"));
    }

    false
}
