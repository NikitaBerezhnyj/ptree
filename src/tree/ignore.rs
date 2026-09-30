use std::path::Path;

pub struct IgnoreMatcher {
    patterns: Vec<String>,
}

impl IgnoreMatcher {
    pub fn new(patterns: &[String]) -> Self {
        Self {
            patterns: patterns.to_vec(),
        }
    }

    pub fn is_ignored(&self, path: &Path) -> bool {
        path.file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| {
                self.patterns
                    .iter()
                    .any(|pattern| matches_pattern(name, pattern))
            })
    }
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
