//! Les différents parser.
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_while_m_n};
use nom::character::complete::{alphanumeric1, multispace0, multispace1};
use nom::sequence::{delimited, preceded, separated_pair, terminated};
use nom::IResult;

/// Reconnais un attribut html.
pub fn attribute(input: &str) -> IResult<&str, (&str, &str)> {
    preceded(
        multispace1,
        alt((
            parse_quoted_attribute,
            parse_monoword_attribute,
            parse_word_attribute,
        )),
    )(input)
}

/// Reconnais un attribut monomots
/// <tag html >
fn parse_word_attribute(input: &str) -> IResult<&str, (&str, &str)> {
    let attribute = alphanumeric1(input);
    match attribute {
        Ok((left, name)) => Ok((left, (name, "yes"))),
        Err(err) => Err(err),
    }
}

/// Reconnais une attribut html avec une valeur sans quote
/// load=async
fn parse_monoword_attribute(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alphanumeric1, equal, alphanumeric1)(input)
}

/// Reconnais un attribut html avec valeur sous quote :
/// class="croquembouche"
fn parse_quoted_attribute(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alphanumeric1, equal, parse_quoted_string)(input)
}

/// Reconnais une chaîne de caractère.
fn parse_quoted_string(input: &str) -> IResult<&str, &str> {
    delimited(quote, is_not("\"'"), quote)(input)
}

pub fn chevron_open(input: &str) -> IResult<&str, &str> {
    preceded(multispace0, tag("<"))(input)
}

pub fn comment_cl(input: &str) -> IResult<&str, &str> {
    terminated(comment_trait, chevron_close)(input)
}

/// Reconnais le début d'un commentaire.
pub fn comment_op(input: &str) -> IResult<&str, &str> {
    preceded(chibang, comment_trait)(input)
}

/// Début de chibang.
pub fn chibang(input: &str) -> IResult<&str, &str> {
    preceded(multispace0, tag("<!"))(input)
}

/// Fin d'un tag fermant space proof.
pub fn chevron_tag_close(input: &str) -> IResult<&str, &str> {
    preceded(multispace0, tag("/>"))(input)
}

pub fn chevron_tag_close_open(input: &str) -> IResult<&str, &str> {
    preceded(multispace0, tag("</"))(input)
}

/// Fin d'un tag ouvrant, espace avant ignore.
pub fn chevron_close(input: &str) -> IResult<&str, &str> {
    preceded(multispace0, tag(">"))(input)
}

fn comment_trait(input: &str) -> IResult<&str, &str> {
    take_while_m_n(2, 100, |ch| ch == '-')(input)
}

/// Reconnais le symbole = pour les attributions.
fn equal(input: &str) -> IResult<&str, &str> {
    delimited(multispace0, tag("="), multispace0)(input)
}

/// Reconnais les délimitations chaîne de caractères : " ou '
fn quote(input: &str) -> IResult<&str, &str> {
    alt((tag("\""), tag("'")))(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::raw_type::{attribute, chevron_close, chibang, equal, parse_quoted_string, quote, comment_op};

    #[test]
    fn atomic_term() {
        assert_eq!(quote("\""), Ok(("", "\"")));
        assert_eq!(quote("'dd"), Ok(("dd", "'")));

        assert_eq!(equal("="), Ok(("", "=")));
    }

    #[test]
    fn open_start_chevron() {
        assert_eq!(chibang(" \t <!ok"), Ok(("ok", "<!")));

        assert_eq!(comment_op("<!--"), Ok(("", "--")));
    }

    #[test]
    fn close_end_chevron() {
        assert_eq!(chevron_close("  >"), Ok(("", ">")));
    }

    #[test]
    fn quoted_string() {
        let test = r#""je suis une chaîne de caractère ├""#;
        assert_eq!(
            parse_quoted_string(test.clone()),
            Ok(("", test.replace("\"", "").as_str()))
        );

        assert_eq!(
            parse_quoted_string(r#"'string!!' ed"#),
            Ok((" ed", "string!!"))
        );
    }

    #[test]
    fn all_attribute() {
        assert_eq!(
            attribute(r#" class="okay boomer""#),
            Ok(("", ("class", "okay boomer")))
        );
        assert_eq!(
            attribute(r#" class = 'okay boomer'"#),
            Ok(("", ("class", "okay boomer")))
        );

        assert_eq!(attribute(" load=async"), Ok(("", ("load", "async"))));

        assert_eq!(attribute(" html"), Ok(("", ("html", "yes"))));
    }
}
