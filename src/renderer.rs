use crate::tree::{NodeKind, TreeNode};

pub struct TreeRenderer;

impl TreeRenderer {
    pub fn render(root: &TreeNode) -> String {
        let mut output = String::new();

        output.push_str(&root.name);
        output.push('\n');

        for (index, child) in root.children.iter().enumerate() {
            let is_last = index == root.children.len() - 1;

            render_node(child, "", is_last, &mut output);
        }

        output
    }
}

fn render_node(node: &TreeNode, prefix: &str, is_last: bool, output: &mut String) {
    let connector = if is_last { "└── " } else { "├── " };

    output.push_str(prefix);
    output.push_str(connector);
    output.push_str(&node.name);

    if matches!(node.kind, NodeKind::Directory) {
        output.push('/');
    }

    output.push('\n');

    if node.children.is_empty() {
        return;
    }

    let child_prefix = if is_last {
        format!("{prefix}    ")
    } else {
        format!("{prefix}│   ")
    };

    for (index, child) in node.children.iter().enumerate() {
        let child_is_last = index == node.children.len() - 1;

        render_node(child, &child_prefix, child_is_last, output);
    }
}
