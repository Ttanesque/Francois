use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[derive(Debug, PartialEq)]
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

pub fn document(doctype: String, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Document(DocumentType { doctype }),
    }
}

pub fn comment(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Comment(data),
    }
}

impl PartialEq for Node {
   fn eq(&self, other: &Self) -> bool {
       self.node_type == other.node_type && self.children == other.children
   } 
}


impl Display for DocumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DocumentType doctype - {}",
            self.doctype
        )
    }
}

impl PartialEq for DocumentType {
    fn eq(&self, other: &Self) -> bool {
        self.doctype == other.doctype
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

impl PartialEq for ElementType {
    fn eq(&self, other: &Self) -> bool {
        self.tag_name == other.tag_name && self.attributes == other.attributes
    }
}
