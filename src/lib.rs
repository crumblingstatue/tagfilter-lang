use parser::{ParseError, Requirement};

mod parser;
mod tokenizer;

/// Parse a text input into a requirement tree
pub fn parse(input: &str) -> Result<Vec<Requirement>, ParseError> {
    let tokens = tokenizer::tokenize(input);
    parser::parse(&tokens)
}

#[test]
fn test_readme_examples() {
    use crate::parser::FnCall;
    assert_eq!(
        parse("bicycle").as_deref(),
        Ok(&[Requirement::Tag("bicycle")][..])
    );
    assert_eq!(
        parse("@any[foo bar] baz").as_deref(),
        Ok(&[
            Requirement::FnCall(FnCall {
                name: "any",
                params: vec![Requirement::Tag("foo"), Requirement::Tag("bar"),]
            }),
            Requirement::Tag("baz")
        ][..])
    );
    assert_eq!(
        parse("@any[cat @all[dog stick]]").as_deref(),
        Ok(&[Requirement::FnCall(FnCall {
            name: "any",
            params: vec![
                Requirement::Tag("cat"),
                Requirement::FnCall(FnCall {
                    name: "all",
                    params: vec![Requirement::Tag("dog"), Requirement::Tag("stick"),]
                })
            ]
        })][..])
    );
    assert_eq!(
        parse("@untagged").as_deref(),
        Ok(&[Requirement::FnCall(FnCall {
            name: "untagged",
            params: vec![],
        })][..])
    );
}
