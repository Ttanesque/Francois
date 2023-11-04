use log::info;

use crate::dom::Node;
use crate::dom::NodeType::{Comment, Document, Element, Root, Text};

pub fn pretty_print(tree: &Node) {
    dom_print(tree, "".to_string())
}

fn dom_print(tree: &Node, pre_line: String) {
    match &tree.node_type {
        Root => info!("{}Root", pre_line),
        Text(data) => info!("{}Text {}", pre_line, data),
        Comment(data) => info!("{}Comment {}", pre_line, data),
        Element(node_spec) => info!("{}{}", pre_line, node_spec),
        Document(node_spec) => info!("{}{}", pre_line, node_spec),
    }

    if tree.children.len() > 0 {
        let mut pre_line_child;
        for (i, node) in tree.children.iter().enumerate() {
            if i == tree.children.len() - 1 {
                pre_line_child = pre_line.clone().replace("├─", "│ ");
            } else {
                pre_line_child = pre_line.clone();
            }

            if tree.children.len() > 1 {
                pre_line_child.push_str("\t├─");
            } else {
                pre_line_child.push_str("\t└─")
            }

            dom_print(node, pre_line_child.clone());
        }
    }
}
