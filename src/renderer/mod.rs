mod compact;
mod json;
mod stats;
mod tree;

use crate::cli::OutputFormat;
use crate::tree::TreeNode;

pub struct Renderer;

impl Renderer {
    pub fn render(root: &TreeNode, format: OutputFormat, show_stats: bool) -> String {
        match format {
            OutputFormat::Tree => {
                if show_stats {
                    stats::render(root)
                } else {
                    tree::render(root)
                }
            }
            OutputFormat::Compact => compact::render(root),
            OutputFormat::Json => json::render(root),
        }
    }
}
