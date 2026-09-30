use super::{NodeKind, TreeNode};

#[derive(Debug)]
pub struct TreeStats {
    pub files: usize,
    pub directories: usize,
}

impl TreeStats {
    pub fn from_tree(root: &TreeNode) -> Self {
        let mut stats = Self {
            files: 0,
            directories: 0,
        };

        collect(root, &mut stats);

        stats
    }

    pub fn file_count(node: &TreeNode) -> usize {
        match node.kind {
            NodeKind::File => 1,
            NodeKind::Directory => node.children.iter().map(Self::file_count).sum(),
        }
    }
}

fn collect(node: &TreeNode, stats: &mut TreeStats) {
    match node.kind {
        NodeKind::File => stats.files += 1,
        NodeKind::Directory => {
            stats.directories += 1;

            for child in &node.children {
                collect(child, stats);
            }
        }
    }
}
