use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Root,
    Text(String),
    Element(ElementType),
    Comment(String),
    Document(DocumentType),
}

#[derive(Debug)]
pub struct ElementType {
    pub tag_name: String,
    pub attributes: AttrMap,
}

#[derive(Debug)]
pub struct DocumentType {
    pub charset: String,
    pub doctype: String,
}

pub type AttrMap = HashMap<String, String>;

pub fn root() -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Root,
    }
}

pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementType {
            tag_name: name,
            attributes: attrs,
        }),
    }
}

pub fn document(doctype: String, charset: String, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Document(DocumentType { charset, doctype }),
    }
}

pub fn comment(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Comment(data),
    }
}

impl Display for DocumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DocumentType doctype - {}, charset - {}",
            self.doctype, self.charset
        )
    }
}

impl Display for ElementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ElementType : tag_name - {}, attributes - {}",
            self.tag_name,
            self.attributes
                .iter()
                .map(|(k, v)| format!("{}_{}", k, v))
                .collect::<Vec<String>>()
                .join(" , ")
        )
    }
}
