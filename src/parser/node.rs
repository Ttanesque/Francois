//! Parser retournant les types du graphs.
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_till},
    character::complete::alphanumeric1,
    error::context,
    multi::many0,
    sequence::{delimited, tuple, terminated},
    IResult,
};

use crate::dom::{comment, document, elem, text, AttrMap, Node, NodeType};

use super::raw_type::{
    attribute, chevron_close, chevron_open, chevron_tag_close, chevron_tag_close_open, chibang,
    comment_cl, comment_op,
};

/// Reconnais un tag avec des enfants.
///
/// # Exemple
///
/// ```
/// use francois::parser::parse_tag;
/// use francois::dom;
///
/// ```
pub fn parse_tag<'a, str: 'a + ?Sized>(input: &str) -> IResult<&str, Node> {
    match p_tag_open(input) {
        Ok((left, node)) => {
            if let NodeType::Element(node_type) = node.node_type {
                let close = p_tag_close(node_type.tag_name.as_str());
                let ok = terminated(p_node, close)(left);
            }
        }
        Err(err) => return Err(err),
    }

    unimplemented!()
}

pub fn p_tag_close(name: &str) -> impl FnMut(&'static str) -> Result<(&'static str, &'static str), nom::Err<nom::error::Error<&'static str>>> + '_ {
    delimited(chevron_tag_close_open, tag(name), chevron_close)
}

/// Parser de l'ouverture d'un tag. Créer une Node avec tout les attributs il ne reste que les
/// enfants à traiter.
///
/// # Exemple
///
/// ```
/// use francois::parser::node::p_tag_open;
/// use francois::dom;
///
/// let mut attrs = dom::AttrMap::new();
/// attrs.insert("class".to_string(), "ok".to_string());
///
/// assert_eq!(
///     p_tag_open(r#"<body class="ok">"#),
///     Ok(("", dom::elem("body".to_string(), attrs, Vec::new())))
/// );
/// ```
pub fn p_tag_open(input: &str) -> IResult<&str, Node> {
    let mut open = context(
        "tag open",
        delimited(
            chevron_open,
            tuple((alphanumeric1, many0(attribute))),
            chevron_close,
        ),
    );
    // let mut close = delimited(chevron_tag_close_open, alphanumeric1, chevron_close);

    match open(input) {
        Ok((left, (tag, attributes))) => {
            let attrs = attributes_to_map(attributes);

            Ok((left, elem(tag.to_string(), attrs, Vec::new())))
        }
        Err(err) => Err(err),
    }
}

/// Reconnais une Node.
///
/// # Exemples
///
/// ```
/// use francois::parser::node::p_node;
/// use francois::dom;
///
/// assert_eq!(p_node(" ceci est un text"), Ok(("", dom::text("ceci est un text".to_string()))));
/// assert_eq!(p_node(
///     "<!-- je suis venu, j'ai vu, j'ai vaincu -->"),
///     Ok(("", dom::comment("je suis venu, j'ai vu, j'ai vaincu".to_string())))
/// );
/// let mut attrs = dom::AttrMap::new();
/// attrs.insert("src".to_string(), "dd".to_string());
/// assert_eq!(p_node(r#"<img src="dd"/>"#), Ok(("", dom::elem("img".to_string(), attrs, Vec::new()))));
// ```
pub fn p_node(input: &str) -> IResult<&str, Node> {
    alt((p_comment, p_tag_autocloseable, p_text_content, parse_tag))(input)
}

/// Reconnais un text.
///
/// # Exemples
///
/// ```
/// use francois::parser::node;
/// use francois::dom;
///
/// assert_eq!(node::p_text_content(" octave est un mouflon. "), Ok(("", dom::text("octave est un mouflon.".to_string()))));
/// ```
pub fn p_text_content(input: &str) -> IResult<&str, Node> {
    match take_till(|chara| chara == '<')(input) {
        Ok((left, data)) => Ok((left, text(data.trim().to_string()))),
        Err(err) => Err(err),
    }
}

/// Reconnais un tag autoferme.
///
/// # Exemple
/// ```
/// use francois::parser::node;
/// use francois::dom;
///
/// let mut map = dom::AttrMap::new();
/// map.insert("type".to_string(), "jpg".to_string());
///
/// match node::p_tag_autocloseable("<img type=jpg />") {
///     Ok((left, node)) => assert_eq!(node, dom::elem("img".to_string(), map, Vec::new())),
///     Err(err) => assert!(false, "{}", err),
/// }
/// ```
pub fn p_tag_autocloseable(input: &str) -> IResult<&str, Node> {
    let mut parser = tuple((
        chevron_open,
        is_not(" >"),
        many0(attribute),
        chevron_tag_close,
    ));

    match parser(input) {
        Ok((left, (_, tagname, attributes, _))) => {
            let attrs = attributes_to_map(attributes);

            Ok((left, elem(tagname.to_string(), attrs, Vec::new())))
        }
        Err(err) => Err(err),
    }
}

/// Reconnais une node de type doctype <!DOCTYPE html>, html on assigne le type s'il n'a aucune
/// valeur, html est utilisé par défaut.
///
/// # Exemple
///
/// ```
/// match francois::parser::node::p_doctype("<!DOCTYPE html>") {
///     Ok((left, node)) => assert_eq!(node, francois::dom::document("html".to_string(), Vec::new())),
///     Err(err) => println!("{}", err),
/// }
/// // whitespace proof
/// match francois::parser::node::p_doctype("<!DOCTYPE html >") {
///     Ok((left, node)) => assert_eq!(node, francois::dom::document("html".to_string(), Vec::new())),
///     Err(err) => assert!(false, "{}", err),
/// }
/// ```
pub fn p_doctype(input: &str) -> IResult<&str, Node> {
    let mut doctype_parser = tuple((chibang, tag("DOCTYPE"), many0(attribute), chevron_close));
    match doctype_parser(input) {
        Ok((left, (_, _, attrs, _))) => {
            let mut a_doctype = "html";
            for (key, value) in attrs {
                if key == "html" && value != "yes" {
                    a_doctype = key;
                }
            }

            let node_doctype = document(a_doctype.to_string(), Vec::new());

            Ok((left, node_doctype))
        }
        Err(err) => Err(err),
    }
}

/// Reconnais un commentaire <!-- gg -->.
///
/// # Exemple
///
/// ```
/// use francois::parser::node;
/// use francois::dom;
///
/// match node::p_comment("<!-- d d -->") {
///     Ok((left, node)) => assert_eq!(node, dom::comment("d d".to_string())),
///     Err(err) => assert!(false, "{}", err),
/// }
/// ```
pub fn p_comment(input: &str) -> IResult<&str, Node> {
    let mut parser = delimited(comment_op, is_not("-"), comment_cl);

    match parser(input) {
        Ok((left, val)) => Ok((left, comment(val.trim().to_string()))),
        Err(err) => Err(err),
    }
}

fn attributes_to_map(attributes: Vec<(&str, &str)>) -> AttrMap {
    let mut attrs = AttrMap::new();

    attributes.iter().for_each(|(k, v)| {
        attrs.insert(k.to_string(), v.to_string());
    });

    attrs
}
