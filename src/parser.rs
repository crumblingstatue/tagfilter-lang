use std::iter::Peekable;

use thiserror::Error;

use crate::tokenizer::Token;

#[derive(Debug, Error, PartialEq)]
pub enum ParseError<'a> {
    #[error("Unexpected token .0")]
    UnexpectedToken(Token<'a>),
    #[error("Mismatch: expected .expected got .got")]
    Mismatch {
        expected: Token<'a>,
        got: Option<Token<'a>>,
    },
    #[error("Unexpected end")]
    UnexpectedEnd,
}

/// A requirement for how to match tags
#[derive(Debug, PartialEq)]
pub enum Requirement<'a> {
    /// Tag requirement like `foo`
    Tag(&'a str),
    /// Function call requirement like `@myfun[param1 param2]`
    FnCall(FnCall<'a>),
}

/// A function call requirement
#[derive(Debug, PartialEq)]
pub struct FnCall<'a> {
    /// The name of the function
    pub name: &'a str,
    /// The list of parameters
    pub params: Vec<Requirement<'a>>,
}

pub fn parse<'tvec, 'src: 'tvec>(
    tokens: &'tvec [Token<'src>],
) -> Result<Vec<Requirement<'src>>, ParseError<'src>> {
    let mut tokens = tokens.iter().peekable();
    let mut reqs = Vec::new();
    while let Some(tok) = tokens.next() {
        match tok {
            Token::FunIdent(name) => {
                reqs.push(Requirement::FnCall(parse_fn_call(name, &mut tokens)?))
            }
            Token::Tag(name) => reqs.push(Requirement::Tag(name)),
            tok @ (Token::LBracket | Token::RBracket) => {
                return Err(ParseError::UnexpectedToken(*tok))
            }
        }
    }
    Ok(reqs)
}

type Iter<'tvec, 'src> = Peekable<std::slice::Iter<'tvec, Token<'src>>>;

fn parse_fn_call<'tvec, 'src: 'tvec>(
    name: &'src str,
    tokens: &mut Iter<'tvec, 'src>,
) -> Result<FnCall<'src>, ParseError<'src>> {
    match tokens.peek() {
        Some(Token::LBracket) => {
            // consume bracket
            tokens.next().unwrap();
            Ok(FnCall {
                name,
                params: parse_fn_params(tokens)?,
            })
        }
        _ => Ok(FnCall {
            name,
            params: Vec::new(),
        }),
    }
}

fn parse_fn_params<'tvec, 'src: 'tvec>(
    tokens: &mut Iter<'tvec, 'src>,
) -> Result<Vec<Requirement<'src>>, ParseError<'src>> {
    let mut reqs = Vec::new();
    loop {
        match tokens.next() {
            Some(tok) => match tok {
                Token::FunIdent(name) => {
                    reqs.push(Requirement::FnCall(parse_fn_call(name, tokens)?))
                }
                Token::Tag(name) => reqs.push(Requirement::Tag(name)),
                Token::LBracket => return Err(ParseError::UnexpectedToken(Token::LBracket)),
                Token::RBracket => return Ok(reqs),
            },
            None => return Err(ParseError::UnexpectedEnd),
        }
    }
}

#[test]
fn test_parse() {
    // foo
    assert_eq!(
        parse(&[Token::Tag("foo")]).as_deref(),
        Ok(&[Requirement::Tag("foo")][..])
    );
    // foo bar baz
    assert_eq!(
        parse(&[Token::Tag("foo"), Token::Tag("bar"), Token::Tag("baz")]).as_deref(),
        Ok(&[
            Requirement::Tag("foo"),
            Requirement::Tag("bar"),
            Requirement::Tag("baz")
        ][..])
    );
    // @special
    assert_eq!(
        parse(&[Token::FunIdent("special")]).as_deref(),
        Ok(&[Requirement::FnCall(FnCall {
            name: "special",
            params: vec![]
        })][..])
    );
    // @any[a b]
    assert_eq!(
        parse(&[
            Token::FunIdent("any"),
            Token::LBracket,
            Token::Tag("a"),
            Token::Tag("b"),
            Token::RBracket
        ])
        .as_deref(),
        Ok(&[Requirement::FnCall(FnCall {
            name: "any",
            params: vec![Requirement::Tag("a"), Requirement::Tag("b"),]
        })][..])
    );
    // @any[a @all[b c @foo[d e]]] foo-tag
    assert_eq!(
        parse(&[
            Token::FunIdent("any"),
            Token::LBracket,
            Token::Tag("a"),
            Token::FunIdent("all"),
            Token::LBracket,
            Token::Tag("b"),
            Token::Tag("c"),
            Token::FunIdent("foo"),
            Token::LBracket,
            Token::Tag("d"),
            Token::Tag("e"),
            Token::RBracket,
            Token::RBracket,
            Token::RBracket,
            Token::Tag("foo-tag"),
        ])
        .as_deref(),
        Ok(&[
            Requirement::FnCall(FnCall {
                name: "any",
                params: vec![
                    Requirement::Tag("a"),
                    Requirement::FnCall(FnCall {
                        name: "all",
                        params: vec![
                            Requirement::Tag("b"),
                            Requirement::Tag("c"),
                            Requirement::FnCall(FnCall {
                                name: "foo",
                                params: vec![Requirement::Tag("d"), Requirement::Tag("e"),]
                            })
                        ]
                    })
                ]
            }),
            Requirement::Tag("foo-tag")
        ][..])
    );
}
