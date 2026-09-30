use std::fs;
use std::io;
use std::path::Path;

use super::{NodeKind, TreeNode, WalkOptions};

const DEFAULT_IGNORES: &[&str] = &[
    ".git",
    "node_modules",
    ".dart_tool",
    "build",
    "dist",
    "coverage",
    ".idea",
    ".vscode",
    "__pycache__",
    "bin",
    "obj",
    "target",
];

pub fn build_tree(root: &Path, options: &WalkOptions) -> io::Result<TreeNode> {
    let name = root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(".")
        .to_owned();

    build_node(root, name, 0, options)
}

fn build_node(
    path: &Path,
    name: String,
    depth: usize,
    options: &WalkOptions,
) -> io::Result<TreeNode> {
    let metadata = fs::metadata(path)?;

    if metadata.is_file() {
        return Ok(TreeNode::file(name, path.to_path_buf()));
    }

    if metadata.is_dir() {
        if options
            .max_depth
            .is_some_and(|max_depth| depth >= max_depth)
        {
            return Ok(TreeNode::directory(name, path.to_path_buf(), Vec::new()));
        }

        let mut children = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let entry_name = entry.file_name().to_string_lossy().into_owned();

            if !options.show_hidden && entry_name.starts_with('.') {
                continue;
            }

            if DEFAULT_IGNORES.contains(&entry_name.as_str()) {
                continue;
            }

            let node = build_node(&entry_path, entry_name, depth + 1, options)?;

            children.push(node);
        }

        children.retain(|node| should_include(node, options));

        children.sort_by(|a, b| match (&a.kind, &b.kind) {
            (NodeKind::Directory, NodeKind::File) => std::cmp::Ordering::Less,
            (NodeKind::File, NodeKind::Directory) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });

        return Ok(TreeNode::directory(name, path.to_path_buf(), children));
    }

    Ok(TreeNode::file(name, path.to_path_buf()))
}

fn should_include(node: &TreeNode, options: &WalkOptions) -> bool {
    match node.kind {
        NodeKind::File => {
            if options.dirs_only {
                return false;
            }

            if options.files_only || !options.extensions.is_empty() {
                return matches_extension(node, options);
            }

            true
        }
        NodeKind::Directory => {
            if options.files_only || !options.extensions.is_empty() {
                !node.children.is_empty()
            } else {
                true
            }
        }
    }
}

fn matches_extension(node: &TreeNode, options: &WalkOptions) -> bool {
    if options.extensions.is_empty() {
        return true;
    }

    node.path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            options
                .extensions
                .iter()
                .any(|expected| expected.eq_ignore_ascii_case(extension))
        })
}
