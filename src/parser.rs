use std::collections::HashMap;

use log::{warn, error, debug};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{
    dom::{comment, document, root, Node, elem, AttrMap},
    tools::pretty_print,
};

#[derive(Parser)]
#[grammar = "html.pest"]
pub struct HTMLParser;

pub fn parse(text_to_parse: String) {
    let file = HTMLParser::parse(Rule::file, &text_to_parse).expect("Erreur de décodage");

    let mut root = root();

    for line in file {
        let parse_line = parse_line(line);
        if parse_line.is_some() {
            root.children.push(parse_line.unwrap());
        }
    }
    pretty_print(&root);
}

fn parse_line(line: Pair<Rule>) -> Option<Node> {
    match line.as_rule() {
        Rule::doctype => {
            let mut charset = "utf8".to_string();
            let mut doctype = "html".to_string();

            for attr in line.into_inner() {
                match attr.as_rule() {
                    Rule::attribute => {
                        let attr_parse_res = parse_attribute(attr);

                        if attr_parse_res.is_ok() {
                            let (key, val) = attr_parse_res.expect("ysk2");
                            match key.as_str() {
                                "html" => doctype = key,
                                "doctype" => doctype = val,
                                "charset" => charset = val,
                                _ => warn!("Attribut {} {} no, reconnu pour doctype", key, val),
                            }
                        } else {
                            error!("Erreur ignoré{}", attr_parse_res.expect_err("ysk2"));
                        }
                    }
                    _ => (),
                }
            }

            Some(document(doctype, charset, Vec::new()))
        }
        Rule::comment => Some(comment(line.into_inner().as_str().to_string())),
        Rule::EOI => None,
        Rule::element_closable => {
            let mut tag_names = "error";
            let mut attr:AttrMap = AttrMap::new(); 

            for pair in line.into_inner() {
                match pair.as_rule() {
                    Rule::element_start => {
                        tag_names = pair.into_inner().next().unwrap().as_str();
                    },
                    Rule::attribute => {
                        let _ = parse_attribute(pair).and_then(|(k, v)| Ok(attr.insert(k, v)));
                    }
                    _ => ()
                }
            }

            Some(elem(tag_names.to_string(), attr, Vec::new()))
        }
        _ => {
            warn!("Erreur règle non prise en compte {}", line.as_str());
            None
        }
    }
}

fn parse_attribute(attribute: Pair<Rule>) -> Result<(String, String), &str> {
    let mut key: &str = "";
    let mut val: &str = "true";

    for pair in attribute.into_inner() {
        match pair.as_rule() {
            Rule::attr_key => key = pair.as_str(),
            Rule::attr_val => val = pair.into_inner().as_str(),
            _ => return Err("Règle non pris en charge"),
        }
    }

    Ok((key.to_string(), val.to_string()))
}
