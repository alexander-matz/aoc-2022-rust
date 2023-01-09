#[allow(dead_code)]
pub mod aoc {
    use std::str::Chars;


    #[derive(Debug)]
    enum Operator {
        Add,
        Mul,
    }

    #[derive(Debug)]
    enum Operand {
        Old,
        Fixed(i32),
    }

    #[derive(Debug)]
    struct Monkey {
        index: u32,
        items: Vec<u32>,
        operator: Operator,
        op1: Operand,
        op2: Operand,
        divisor: i32,
        target_true: u32,
        target_false: u32,
    }

    #[derive(Debug, Clone)]
    enum Pattern<'a> {
        Literal(&'a str),
        Whitespace,
        Word,
        Sequence(Vec<Pattern<'a>>),
        Many(Box<Pattern<'a>>),
        Eof,
    }

    #[derive(Debug, PartialEq)]
    enum Capture {
        NoCapture,
        One(String),
        Many(Vec<Capture>),
    }

    impl From<&str> for Capture { fn from(value: &str) -> Self { Capture::One(value.to_owned()) } }
    impl From<Vec::<&str>> for Capture {
        fn from(value: Vec::<&str>) -> Self {
            let items: Vec<Capture> = value.iter().map(|str_ref| Capture::One((*str_ref).to_owned())).collect();
            Capture::Many(items)
        }
    }

    #[derive(Debug, Clone)]
    struct ParseError { msg: String }

    impl ParseError {
        fn new(msg: &str) -> ParseError { ParseError{ msg: msg.to_owned() } }
        fn default() -> ParseError { ParseError{ msg: "".to_owned() } }
    }

    impl PartialEq for ParseError { fn eq(&self, _: &Self) -> bool { true } }

    fn is_whitespace(ch: Option<&char>) -> bool {
        match ch {
            Some(&value) => value.is_whitespace(),
            None => false
        }
    }

    fn is_alphanum(ch: Option<&char>) -> bool {
        match ch {
            Some(&value) => value.is_alphanumeric(),
            None => false
        }
    }

    fn parse_pattern_aux(pattern: &Pattern, input_raw: &mut Chars) -> Result<Capture, ParseError> {
        let mut input = input_raw.by_ref().peekable();
        match pattern {
            Pattern::Literal(text) => {
                for ch in text.chars() {
                    if input.peek() != Some(&ch) {
                        return Err(ParseError{ msg: format!("expected '{}' but got '{:?}' for literal '{}'", ch, input.peek(), text)})
                    }
                    input.next();
                }
                Ok(Capture::NoCapture)
            },
            Pattern::Whitespace => {
                match input.peek() {
                    None => return Err(ParseError{ msg: format!("expected whitespace but got '{:?}'", input.peek()) }),
                    Some(&ch) => {
                        if ! ch.is_whitespace() {
                            return Err(ParseError{ msg: format!("expected whitespace but got '{:?}'", input.peek()) })
                        }
                        input.next();
                    }
                }
                while let Some(ch) = input.peek() {
                    if ch.is_whitespace() {
                        input.next();
                    } else {
                        break;
                    }
                }
                Ok(Capture::NoCapture)
            },
            Pattern::Word => {
                let mut buf: Vec<char> = Vec::new();
                match input.peek() {
                    None => return Err(ParseError{ msg: format!("expected alphanum but got '{:?}'", input.peek()) }),
                    Some(&ch) => {
                        if ! ch.is_alphanumeric() {
                            return Err(ParseError{ msg: format!("expected alphanum but got '{:?}'", input.peek()) })
                        }
                        buf.push(ch);
                        input.next();
                    }
                }
                while let Some(&ch) = input.peek() {
                    if ch.is_alphanumeric() {
                        buf.push(ch);
                        input.next();
                    } else {
                        break;
                    }
                }
                Ok(Capture::One(buf.into_iter().collect()))
            },
            Pattern::Sequence(elements) => {
                let mut captures: Vec<Capture> = Vec::new();
                for element in elements {
                    let capture = parse_pattern_aux(element, input_raw)?;
                    captures.push(capture);
                }
                Ok(Capture::Many(captures))
            },
            Pattern::Many(contained_pattern) => {
                let mut captures: Vec<Capture> = Vec::new();
                let first = parse_pattern_aux(&*contained_pattern, input_raw)?;
                captures.push(first);
                loop {
                    let copy = input_raw.clone();
                    let repetition = parse_pattern_aux(&*contained_pattern, input_raw);
                    match repetition {
                        Ok(capture) => { captures.push(capture); },
                        Err(_) => {
                            *input_raw = copy;
                            break;
                        }
                    }
                }
                let filtered: Vec<Capture> = captures.into_iter()
                    .filter(|capture| match capture {
                        Capture::NoCapture => false,
                        _ => true
                    }).collect();
                if filtered.is_empty() {
                    Ok(Capture::NoCapture)
                } else {
                    Ok(Capture::Many(filtered))
                }
            },
            Pattern::Eof => {
                match input.peek() {
                    None => Ok(Capture::NoCapture),
                    Some(ch) => Err(ParseError{ msg: format!("Expected EOF but got '{}'", ch)})
                }
            },
        }
    }

    fn parse_pattern<'a>(pattern: &Pattern, input: &'a str) -> Result<(Capture, &'a str), ParseError> {
        let mut input_chars = input.chars();
        match parse_pattern_aux(pattern, &mut input_chars) {
            Ok(capture) => {
                let rest = input_chars.as_str();
                Ok((capture, rest))
            },
            Err(err) => Err(err),
        }

    }

    const MONKEY_TEXT: &str = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3

  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0";


    #[allow(dead_code)]
    pub fn day_main_part() -> std::io::Result<()> {
        // let monkey_text = input_all();
        Ok(())
    }

    #[allow(non_upper_case_globals, dead_code)]
    pub const day_main: fn() -> std::io::Result<()> = day_main_part;

    #[cfg(test)]
    mod tests {
        // Note this useful idiom: importing names from outer (for mod tests) scope.
        use super::*;

        use Capture::*;
        use Pattern::*;

        // Literal(&'a str),
        // ManyWhitespace,
        // ManyAlphanum,
        // Sequence(Vec<Pattern<'a>>),
        // Many(Box<Pattern<'a>>),
        // Eof,

        #[test]
        fn test_pattern_whitespace() {

            assert_eq!(
                parse_pattern(&Whitespace, " "),
                Ok((NoCapture, ""))
            );

            assert_eq!(
                parse_pattern(&Whitespace, "\n  "),
                Ok((NoCapture, ""))
            );

            assert_eq!(
                parse_pattern(&Whitespace, ""),
                Err(ParseError::default())
            );

            assert_eq!(
                parse_pattern(&Whitespace, "asdf"),
                Err(ParseError::default())
            );
        }

        #[test]
        fn test_pattern_alphanum() {

            assert_eq!(
                parse_pattern(&Word, ""),
                Err(ParseError::default())
            );

            assert_eq!(
                parse_pattern(&Word, " "),
                Err(ParseError::default())
            );

            assert_eq!(
                parse_pattern(&Word, " 123asdf"),
                Err(ParseError::default())
            );

            assert_eq!(
                parse_pattern(&Word, "1"),
                Ok((Capture::from("1"), ""))
            );

            assert_eq!(
                parse_pattern(&Word, "a1"),
                Ok((Capture::from("a1"), ""))
            );

            assert_eq!(
                parse_pattern(&Word, "1a "),
                Ok((Capture::from("1a"), " "))
            );

            assert_eq!(
                parse_pattern(&Word, "1a, "),
                Ok((Capture::from("1a"), ", "))
            );
        }
    }
}