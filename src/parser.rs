#[allow(dead_code)]
use std::rc::Rc;
use std::str::Chars;
use core::fmt::Debug;

#[derive(Debug)]
pub struct ParseError {
    msg: String
}

#[derive(Debug, PartialEq, Clone)]
pub enum Captured {
    None,
    One(String),
    Many(Vec<Captured>),
}

impl Captured {
    pub fn as_one(&self) -> &str {
        match self {
            Captured::One(value) => &value,
            other => panic!("Expected Captured::One, but got {:?}", other),
        }
    }

    pub fn as_many(&self) -> &Vec<Captured> {
        match self {
            Captured::Many(value) => &value,
            other => panic!("Expected Captured::Many, but got {:?}", other),
        }
    }
}

impl From<&str> for Captured {
    fn from(value: &str) -> Self {
        Captured::One(value.to_owned())
    }
}

pub type ParseResult<'a> = Result<(Chars<'a>, Captured), ParseError>;

pub trait Parser {
    fn parse<'a>(&self, input: &'a str, chars: Chars<'a>) -> ParseResult<'a>;
}

impl Debug for dyn Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{dyn Parser}}")
    }
}

#[derive(Debug)]
pub struct Char {
    ch: char
}

impl Parser for Char {
    fn parse<'a>(&self, _: &'a str, mut chars: Chars<'a>) -> ParseResult<'a> {
        match chars.next() {
            None => Err(ParseError{ msg: format!("Expected char '{}', but got EOF", self.ch)}),
            Some(ch) if ch != self.ch =>
                Err(ParseError{ msg: format!("Expected char '{}', but got '{}'", self.ch, ch)}),
            Some(_) => Ok((chars, Captured::None))
        }
    }
}

#[derive(Debug)]
pub struct Range {
    lower: char,
    upper: char,
}

impl Parser for Range {
    fn parse<'a>(&self, _: &'a str, mut chars: Chars<'a>) -> ParseResult<'a> {
        match chars.next() {
            None => Err(ParseError{ msg: format!("Expected char between '{}' and '{}' (both inclusive), but got EOF", self.lower, self.upper)}),
            Some(ch) if ch < self.lower || ch > self.upper =>
                Err(ParseError{ msg: format!("Expected char between '{}' and '{}' (both inclusive), but got '{}'", self.lower, self.upper, ch)}),
            Some(_) => Ok((chars, Captured::None))
        }
    }
}

#[derive(Debug)]
pub struct Alt {
    left: Rc<dyn Parser>,
    right: Rc<dyn Parser>,
}

impl Parser for Alt {
    fn parse<'a>(&self, input: &'a str, chars: Chars<'a>) -> ParseResult<'a> {
        match self.left.parse(input, chars.clone()) {
            Err(_) => (),
            Ok(x) => { return Ok(x) },
        }
        match self.right.parse(input, chars) {
            Err(ParseError{ msg }) => Err(ParseError{ msg: msg + " (right hand side of alternative)" }),
            Ok(x) => Ok(x)
        }
    }
}

#[derive(Debug)]
pub struct AlphaNum {}

impl Parser for AlphaNum {
    fn parse<'a>(&self, _: &'a str, mut chars: Chars<'a>) -> ParseResult<'a> {
        match chars.next() {
            None => Err(ParseError { msg: format!("Expected alphanum character, but got EOF") }),
            Some(ch) if ch.is_alphanumeric() => Ok((chars, Captured::None)),
            Some(ch) => Err(ParseError { msg: format!("Expected alphanum character, but got '{}'", ch) })
        }
    }
}

#[derive(Debug)]
pub struct Lit {
    text: String,
}

impl Parser for Lit {
    fn parse<'a>(&self, _: &'a str, mut chars: Chars<'a>) -> ParseResult<'a> {
        for expected in self.text.chars() {
            let actual = chars.next();
            if actual != Some(expected) {
                return Err(ParseError{ msg: format!("expected '{}' but got '{:?}' for literal '{}'", expected, actual, self.text)})
            }
        }
        Ok((chars, Captured::None))
    }
}

#[derive(Debug)]
pub struct Capture {
    value: Rc<dyn Parser>,
}

impl Parser for Capture {
    fn parse<'a>(&self, input: &'a str, chars: Chars<'a>) -> ParseResult<'a> {
        let start_index = input.len() - chars.as_str().len();
        match self.value.parse(input, chars) {
            Err(err) => {
                Err(err)
            },
            Ok((done_chars, Captured::None)) => {
                let end_index = input.len() - done_chars.as_str().len();
                let capture = &input[start_index .. end_index];
                Ok((done_chars, Captured::One(capture.to_owned())))
            },
            Ok((_, _)) => {
                return Err(ParseError{ msg: format!("Cannot nest Capture parser")})
            }
        }
    }
}

#[derive(Debug)]
pub struct Opt {
    value: Rc<dyn Parser>
}

impl Parser for Opt {
    fn parse<'a>(&self, input: &'a str, chars: Chars<'a>) -> ParseResult<'a> {
        match self.value.parse(input, chars.clone()) {
            Err(_) => {
                return Ok((chars, Captured::None))
            },
            ok => ok
        }
    }
}

#[derive(Debug)]
pub struct Any {
    value: Rc<dyn Parser>
}

impl Parser for Any {
    fn parse<'a>(&self, input: &'a str, mut chars: Chars<'a>) -> ParseResult<'a> {
        let mut many_captures: Vec<Captured> = Vec::new();
        let mut done = false;
        while !done {
            let checkpoint = chars.clone();

            (chars, done) = match self.value.parse(input, chars) {
                Err(_) => {
                    (checkpoint, true)
                },
                Ok((chars, captured)) => {
                    match captured {
                        Captured::None => {},
                        x => many_captures.push(x)
                    }
                    (chars, false)
                }
            };
        }

        Ok((chars, flatten_captures_variable(many_captures)))
    }
}

#[derive(Debug)]
pub struct Many {
    value: Rc<dyn Parser>
}

impl Parser for Many {
    fn parse<'a>(&self, input: &'a str, mut chars: Chars<'a>) -> ParseResult<'a> {
        let mut many_captures: Vec<Captured> = Vec::new();
        chars = match self.value.parse(input, chars) {
            Err(err) => { return Err(err); },
            Ok((chars, captured)) => {
                match captured {
                    Captured::None => {},
                    x => many_captures.push(x)
                }
                chars
            }
        };

        let mut done = false;
        while !done {
            let checkpoint = chars.clone();

            (chars, done) = match self.value.parse(input, chars) {
                Err(_) => {
                    (checkpoint, true)
                },
                Ok((chars, captured)) => {
                    match captured {
                        Captured::None => {},
                        x => many_captures.push(x)
                    }
                    (chars, false)
                }
            };
        }

        Ok((chars, flatten_captures_variable(many_captures)))
    }
}

#[derive(Debug)]
pub struct Seq {
    value: Vec<Rc<dyn Parser>>
}

impl Parser for Seq {
    fn parse<'a>(&self, input: &'a str, mut chars: Chars<'a>) -> ParseResult<'a> {
        let mut captures: Vec<Captured> = Vec::new();
        for parser in self.value.iter() {
            chars = match parser.parse(input, chars) {
                Err(err) => { return Err(err); },
                Ok((chars, captured)) => {
                    match captured {
                        Captured::None => {},
                        x => captures.push(x),
                    }
                    chars
                }
            }
        }
        Ok((chars, flatten_captures_fixed(captures)))
    }
}

#[derive(Debug)]
pub struct List {
    element: Rc<dyn Parser>,
    separator: Rc<dyn Parser>,
}

impl Parser for List {
    fn parse<'a>(&self, input: &'a str, mut chars: Chars<'a>) -> ParseResult<'a> {
        let mut captures: Vec<Captured> = Vec::new();
        chars = match self.element.parse(input, chars) {
            Err(err) => { return Err(err) },
            Ok((chars, capture)) => {
                captures.push(capture);
                chars
            }
        };
        loop {
            chars = match self.separator.parse(input, chars.clone()) {
                Err(_) => break,
                Ok((chars, _)) => chars,
            };
            chars = match self.element.parse(input, chars) {
                Err(err) => { return Err(err) },
                Ok((chars, capture)) => {
                    captures.push(capture);
                    chars
                }
            }
        };
        Ok((chars, flatten_captures_variable(captures)))
    }
}

pub fn make_char(ch: char) -> Rc<dyn Parser> {
    Rc::new(Char{ ch })
}

pub fn make_range(lower: char, upper: char) -> Rc<dyn Parser> {
    assert!(lower < upper);
    Rc::new(Range{ lower, upper })
}

pub fn make_alphanum() -> Rc<dyn Parser> {
    Rc::new(AlphaNum{})
}

pub fn make_alt(left: Rc<dyn Parser>, right: Rc<dyn Parser>) -> Rc<Alt>
{
    Rc::new(Alt{ left, right })
}

pub fn make_ws() -> Rc<dyn Parser> {
    make_alt(
        make_char(' '),
        make_alt( make_char('\t') , make_char('\n'))
    )
}

pub fn make_lit(text: &str) -> Rc<dyn Parser> {
    Rc::new(Lit{ text: text.to_owned() })
}

pub fn make_capture(value: Rc<dyn Parser>) -> Rc<dyn Parser> {
    Rc::new(Capture{ value })
}

pub fn make_opt(value: Rc<dyn Parser>) -> Rc<dyn Parser> {
    Rc::new(Opt{ value })
}

pub fn make_any(value: Rc<dyn Parser>) -> Rc<dyn Parser> {
    Rc::new(Any{ value })
}

pub fn make_many(value: Rc<dyn Parser>) -> Rc<dyn Parser> {
    Rc::new(Many{ value })
}

pub fn make_word() -> Rc<dyn Parser> {
    make_many(make_alphanum())
}

pub fn make_number() -> Rc<dyn Parser> {
    make_seq(vec![
        make_opt(make_char('-')),
        make_many(make_range('0', '9'))
    ])
}

pub fn make_seq(value: Vec<Rc<dyn Parser>>) -> Rc<dyn Parser> {
    Rc::new(Seq { value: value })
}

pub fn make_list(element: Rc<dyn Parser>, separator: Rc<dyn Parser>) -> Rc<dyn Parser>
{
    Rc::new(List{ element, separator })
}

fn flatten_captures_variable(maybe_captures: Vec<Captured>) -> Captured {
    if maybe_captures.len() > 0 {
        Captured::Many(maybe_captures)
    } else {
        Captured::None
    }
}

fn flatten_captures_fixed(maybe_captures: Vec<Captured>) -> Captured {
    if maybe_captures.len() == 0 {
        Captured::None
    } else if maybe_captures.len() == 1 {
        maybe_captures.first().unwrap().clone()
    } else {
        Captured::Many(maybe_captures)
    }
}

pub fn parse_wildcard<'a>(pattern: &str, wildcard: char, text: &'a str) -> Option<Vec<&'a str>> {
    let mut captures = Vec::new();
    let mut pc = pattern.chars();
    let mut tc = text.chars();
    while let Some(expected) = pc.next() {
        if expected == wildcard {
            let pc_copy = pc.clone();
            let next_expected = pc.next();
            assert!(next_expected != Some(wildcard));
            let capture_start = text.len() - tc.as_str().len();
            loop {
                let tc_copy = tc.clone();
                match tc.next() {
                    x if x == next_expected => {
                        tc = tc_copy;
                        break
                    }
                    None =>  break,
                    Some(_) => (),
                }
            }
            let capture_end = text.len() - tc.as_str().len();
            pc = pc_copy;
            captures.push(&text[capture_start..capture_end]);
        } else {
            if Some(expected) != tc.next() {
                return None
            }
        }
    }
    Some(captures)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_text<'a>(parser: Rc<dyn Parser>, text: &'a str) -> ParseResult<'a> {
        parser.parse(text, text.chars())
    }

    #[test]
    fn test_parser_lit() {

        let parser = make_lit("1a");

        assert_matches!(
            parse_text(parser.clone(), "1a"),
            Ok((remain, Captured::None)) if remain.as_str() == ""
        );

        assert_matches!(
            parse_text(parser.clone(), "1a-extra"),
            Ok((remain, Captured::None)) if remain.as_str() == "-extra"
        );

        assert_matches!(
            parse_text(parser.clone(), "extra-1a"),
            Err(_)
        );
    }

    #[test]
    fn test_parser_cap(){
        let parser = make_capture( make_lit("1a") );

        assert_matches!(
            parse_text(parser.clone(), "1a-extra"),
            Ok((remain, Captured::One(captured))) if remain.as_str() == "-extra" && captured == "1a"
        );

        assert_matches!(
            parse_text(parser.clone(), "a"),
            Err(_)
        );
    }

    #[test]
    fn test_parser_many() {
        assert_matches!(
            parse_text(make_many( make_lit(" ") ), " "),
            Ok((remain, Captured::None)) if remain.as_str() == ""
        );

        assert_matches!(
            parse_text(make_many( make_lit(" ") ), "     "),
            Ok((remain, Captured::None)) if remain.as_str() == ""
        );

        assert_matches!(
            parse_text(make_many( make_lit(" ") ), "     a"),
            Ok((remain, Captured::None)) if remain.as_str() == "a"
        );

        assert_matches!(
            parse_text(make_many( make_lit(" ") ), "a"),
            Err(_)
        );

        assert_matches!(
            parse_text(make_many( make_capture( make_lit( "1" ) ) ), "111"),
            Ok((remain, Captured::Many(captures))) if remain.as_str() == "" &&
                captures == vec![Captured::from("1"), Captured::from("1"), Captured::from("1")]
        );
    }

    #[test]
    fn test_parser_word() {
        assert_matches!(
            parse_text(make_word(), "asdf321"),
            Ok((remain, Captured::None)) if remain.as_str() == ""
        );

        assert_matches!(
            parse_text(make_word(), "asdf, "),
            Ok((remain, Captured::None)) if remain.as_str() == ", "
        );
    }

    #[test]
    fn test_parser_seq() {
        let parser = make_seq(vec!(
            make_lit("pre "),
            make_capture( make_lit("middle") ),
            make_lit(" post")
        ));

        assert_matches!(
            parse_text(parser, "pre middle post"),
            Ok((remain, Captured::One(capture))) if remain.as_str() == "" && capture == "middle"
        );
    }

    #[test]
    fn test_parser_list() {
        let parser = make_list(
            make_capture( make_word() ),
            make_lit(", ")
        );

        assert_matches!(
            parse_text(parser.clone(), "123"),
            Ok((remain, Captured::Many(capture))) if remain.as_str() == ""
                && capture == vec![Captured::from("123")]
        );

        assert_matches!(
            parse_text(parser.clone(), "123!!"),
            Ok((remain, Captured::Many(capture))) if remain.as_str() == "!!"
                && capture == vec![Captured::from("123")]
        );

        assert_matches!(
            parse_text(parser.clone(), "123, "),
            Err(_)
        );

        assert_matches!(
            parse_text(parser.clone(), "123, 321"),
            Ok((remain, Captured::Many(captures))) if remain.as_str() == ""
                && captures == vec![Captured::from("123"), Captured::from("321")]
        );

        assert_matches!(
            parse_text(parser.clone(), ", 123, "),
            Err(_)
        );
    }

    #[test]
    fn test_parse_wildcard() {
        assert_eq!(
            parse_wildcard("asdf x=*, y=*", '*', "asdf x=-123, y=321"),
            Some(vec!["-123", "321"])
        );

        assert_eq!(
            parse_wildcard("asdf x=*, y=*", '*', "sdf x=-123, y=321"),
            None
        );
    }

}