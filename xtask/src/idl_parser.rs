use winnow::{
    ModalResult, Parser,
    combinator::{alt, repeat},
    error::{ContextError, ErrMode},
    token::{literal, none_of, take_till, take_until, take_while},
};

pub fn parse_idl(input: &mut &'static str) -> Result<Vec<(String, String)>, ErrMode<ContextError>> {
    repeat::<&str, (String, String), Vec<(String, String)>, _, _>(0.., parsers).parse_next(input)
}

fn parsers(input: &mut &str) -> ModalResult<(String, String)> {
    (to_dictionary, dictionary, ws, ident, to_braces, braces)
        .map(|o| (o.3, o.5))
        .parse_next(input)
}

fn to_dictionary(input: &mut &str) -> ModalResult<String> {
    take_until(1.., "dictionary ")
        .parse_next(input)
        .map(|s| s.to_string())
}

fn to_braces(input: &mut &str) -> ModalResult<String> {
    take_until(1.., "{")
        .parse_next(input)
        .map(|s| s.to_string())
}

fn dictionary(input: &mut &str) -> ModalResult<String> {
    literal("dictionary")
        .parse_next(input)
        .map(|s| s.to_string())
}

fn braces(input: &mut &str) -> ModalResult<String> {
    (
        literal("{"),
        repeat(
            0..,
            alt((none_of(['{', '}']).map(|c: char| c.to_string()), braces)),
        ),
        literal("}"),
    )
        .parse_next(input)
        .map(|(_, inner, _): (_, Vec<String>, _)| format!("{{{}}}", inner.join("")))
}

fn ident(input: &mut &str) -> ModalResult<String> {
    take_till(0.., WS).parse_next(input).map(|s| s.to_string())
}

const WS: &[char] = &[' ', '\t', '\r', '\n'];
fn ws(input: &mut &str) -> ModalResult<String> {
    take_while(0.., WS).parse_next(input).map(|s| s.to_string())
}
