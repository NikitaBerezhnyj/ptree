use std::path::Path;

use crate::tree::{NodeKind, TreeNode};

pub fn render(root: &TreeNode) -> String {
    let mut output = String::new();

    render_children(root, Path::new(""), &mut output);

    output
}

fn render_children(node: &TreeNode, parent_path: &Path, output: &mut String) {
    for child in &node.children {
        render_node(child, parent_path, output);
    }
}

fn render_node(node: &TreeNode, parent_path: &Path, output: &mut String) {
    let path = parent_path.join(&node.name);

    output.push_str(&path.to_string_lossy().replace('\\', "/"));

    if matches!(node.kind, NodeKind::Directory) {
        output.push('/');
        output.push('\n');

        render_children(node, &path, output);
    } else {
        output.push('\n');
    }
}
