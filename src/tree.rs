use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum NodeKind {
    Directory,
    File,
}

#[derive(Debug)]
pub struct TreeNode {
    pub name: String,
    pub path: PathBuf,
    pub kind: NodeKind,
    pub children: Vec<TreeNode>,
}

pub struct WalkOptions {
    pub max_depth: Option<usize>,
}

impl TreeNode {
    fn directory(name: String, path: PathBuf, children: Vec<TreeNode>) -> Self {
        Self {
            name,
            path,
            kind: NodeKind::Directory,
            children,
        }
    }

    fn file(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            kind: NodeKind::File,
            children: Vec::new(),
        }
    }
}

pub fn build_tree(root: &Path, max_depth: Option<usize>) -> io::Result<TreeNode> {
    let name = root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(".")
        .to_owned();

    build_node(root, name, 0, max_depth)
}

fn build_node(
    path: &Path,
    name: String,
    depth: usize,
    max_depth: Option<usize>,
) -> io::Result<TreeNode> {
    let metadata = fs::metadata(path)?;

    if metadata.is_file() {
        return Ok(TreeNode::file(name, path.to_path_buf()));
    }

    if metadata.is_dir() {
        if max_depth.is_some_and(|max_depth| depth >= max_depth) {
            return Ok(TreeNode::directory(name, path.to_path_buf(), Vec::new()));
        }

        let mut children = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            let entry_name = entry.file_name().to_string_lossy().into_owned();

            children.push(build_node(&entry_path, entry_name, depth + 1, max_depth)?);
        }

        children.sort_by(|a, b| match (&a.kind, &b.kind) {
            (NodeKind::Directory, NodeKind::File) => std::cmp::Ordering::Less,
            (NodeKind::File, NodeKind::Directory) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });

        return Ok(TreeNode::directory(name, path.to_path_buf(), children));
    }

    Ok(TreeNode::file(name, path.to_path_buf()))
}
