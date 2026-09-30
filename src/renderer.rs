use crate::tree::{NodeKind, TreeNode, TreeStats};

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

    pub fn render_stats(root: &TreeNode) -> String {
        let stats = TreeStats::from_tree(root);

        let mut output = String::from("Project structure\n\n");

        for child in &root.children {
            render_stats_node(child, "", &mut output);
        }

        output.push_str("\n────────────────────────────\n");
        output.push_str(&format!("Files: {:>25}\n", stats.files));
        output.push_str(&format!("Directories: {:>19}\n", stats.directories - 1));

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

fn render_stats_node(node: &TreeNode, prefix: &str, output: &mut String) {
    match node.kind {
        NodeKind::Directory => {
            let count = TreeStats::file_count(node);

            output.push_str(&format!("{prefix}{}/ {:>20} files\n", node.name, count));

            for (index, child) in node.children.iter().enumerate() {
                let is_last = index == node.children.len() - 1;
                render_stats_child(child, &format!("{prefix}"), is_last, output);
            }
        }
        NodeKind::File => {
            output.push_str(&format!("{prefix}{}\n", node.name));
        }
    }
}

fn render_stats_child(node: &TreeNode, prefix: &str, is_last: bool, output: &mut String) {
    let connector = if is_last { "└── " } else { "├── " };

    match node.kind {
        NodeKind::File => {
            output.push_str(&format!("{prefix}{connector}{}\n", node.name));
        }
        NodeKind::Directory => {
            let count = TreeStats::file_count(node);

            output.push_str(&format!(
                "{prefix}{connector}{}/ {:>15} files\n",
                node.name, count
            ));

            let child_prefix = if is_last {
                format!("{prefix}    ")
            } else {
                format!("{prefix}│   ")
            };

            for (index, child) in node.children.iter().enumerate() {
                let child_is_last = index == node.children.len() - 1;
                render_stats_child(child, &child_prefix, child_is_last, output);
            }
        }
    }
}
