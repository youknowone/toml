use crate::key::Key;
use crate::parser::strings::{basic_string, literal_string};
use crate::parser::trivia::ws;
use crate::repr::{InternalString, Repr};
use combine::parser::char::char;
use combine::parser::range::{recognize_with_value, take_while1};
use combine::stream::RangeStream;
use combine::*;

// key = simple-key / dotted-key
// dotted-key = simple-key 1*( dot-sep simple-key )
parse!(key() -> Vec<Key>, {
    sep_by1(between(ws(), ws(), simple_key().map(|(raw, key)| Key::new(Repr::new_unchecked(raw), key))),
            char(DOT_SEP))
});

// simple-key = quoted-key / unquoted-key
parse!(simple_key() -> (&'a str, InternalString), {
    recognize_with_value(choice((
        quoted_key(),
        unquoted_key().map(|s: &'a str| s.into()),
    )))
});

// unquoted-key = 1*( ALPHA / DIGIT / %x2D / %x5F ) ; A-Z / a-z / 0-9 / - / _
parse!(unquoted_key() -> &'a str, {
    take_while1(is_unquoted_char)
});

#[inline]
fn is_unquoted_char(c: char) -> bool {
    matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_')
}

// quoted-key = basic-string / literal-string
parse!(quoted_key() -> InternalString, {
    choice((
        basic_string(),
        literal_string().map(|s: &'a str| s.into()),
    ))
});

// dot-sep   = ws %x2E ws  ; . Period
const DOT_SEP: char = '.';
