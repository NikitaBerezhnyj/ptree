use serde::Serialize;

use crate::tree::{NodeKind, TreeNode};

#[derive(Serialize)]
struct JsonOutput {
    entries: Vec<JsonEntry>,
}

#[derive(Serialize)]
struct JsonEntry {
    path: String,
    #[serde(rename = "type")]
    kind: &'static str,
}

pub fn render(root: &TreeNode) -> String {
    let mut entries = Vec::new();

    for child in &root.children {
        render_node(child, "", &mut entries);
    }

    let output = JsonOutput { entries };

    serde_json::to_string_pretty(&output).expect("JSON serialization should not fail")
}

fn render_node(node: &TreeNode, parent_path: &str, entries: &mut Vec<JsonEntry>) {
    let path = if parent_path.is_empty() {
        node.name.clone()
    } else {
        format!("{parent_path}/{}", node.name)
    };

    let kind = match node.kind {
        NodeKind::Directory => "directory",
        NodeKind::File => "file",
    };

    entries.push(JsonEntry {
        path: path.clone(),
        kind,
    });

    if matches!(node.kind, NodeKind::Directory) {
        for child in &node.children {
            render_node(child, &path, entries);
        }
    }
}
