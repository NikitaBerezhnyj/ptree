use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub enum NodeKind {
    Directory,
    File,
}

#[derive(Debug, Serialize)]
pub struct TreeNode {
    pub name: String,
    pub path: PathBuf,
    pub kind: NodeKind,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn directory(name: String, path: PathBuf, children: Vec<TreeNode>) -> Self {
        Self {
            name,
            path,
            kind: NodeKind::Directory,
            children,
        }
    }

    pub fn file(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            kind: NodeKind::File,
            children: Vec::new(),
        }
    }
}
