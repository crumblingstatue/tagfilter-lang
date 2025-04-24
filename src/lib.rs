#![warn(missing_docs, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![doc = include_str!("../README.md")]

mod parser;
mod tokenizer;

pub use parser::{ParseError, Requirement};

/// Parse a text input into a requirement tree
pub fn parse(input: &str) -> Result<Vec<Requirement>, ParseError> {
    let tokens = tokenizer::tokenize(input);
    parser::parse(&tokens)
}

#[test]
fn test_readme_examples() {
    use crate::parser::{FnCall as Fc, Requirement::*};

    macro_rules! examples {
        ($($input:literal => $expected:expr)*) => {
            $(
                assert_eq!(parse($input).as_deref(), Ok(&$expected[..]));
            )*
        };
    }
    macro_rules! fc {
        ($name:literal$(,)? $($param:expr$(,)?)*) => {
            FnCall(Fc{name: $name, params: vec![$($param,)*]})
        };
    }

    examples! {
        // == tag examples ==

        "hello-world" => [Tag("hello-world")]
        "hello@i@am@a@tag" => [Tag("hello@i@am@a@tag")]
        "brick(character)" => [Tag("brick(character)")]
        "2" => [Tag("2")]

        // == negation examples ==

        // Matches anything that isn't tagged foo
        "!foo" => [Not(Box::new(Tag("foo")))]
        // Matches anything that isn't tagged both foo and bar
        "!@all[foo bar]" => [Not(Box::new(fc!("all", Tag("foo"), Tag("bar"))))]

        // == main examples at the end ==

        // Matches the tag bicycle
        "bicycle" => [Tag("bicycle")]
        // Matches everything that isn't tagged bicycle
        "!bicycle" => [Not(Box::new(Tag("bicycle")))]
        // Must match either foo or bar, and also it has to match baz
        "@any[foo bar] baz" => [
            fc!("any", Tag("foo"), Tag("bar")),
            Tag("baz")
        ]
        // Matches either a cat, or a dog with a stick
        "@any[cat @all[dog stick]]" => [fc!("any", Tag("cat"), fc!("all", Tag("dog"), Tag("stick")))]
        // Matches things that are not tagged
        "@untagged" => [fc!("untagged")]
    }
}
