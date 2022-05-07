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
    /// Negate the results of the child requirement
    Not(Box<Requirement<'a>>),
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
    while tokens.peek().is_some() {
        reqs.push(parse_requirement(&mut tokens)?);
    }
    Ok(reqs)
}

type Iter<'tvec, 'src> = Peekable<std::slice::Iter<'tvec, Token<'src>>>;

fn parse_requirement<'tvec, 'src: 'tvec>(
    tokens: &mut Iter<'tvec, 'src>,
) -> Result<Requirement<'src>, ParseError<'src>> {
    match tokens.next() {
        Some(tok) => match tok {
            Token::FunIdent(name) => Ok(Requirement::FnCall(parse_fn_call(name, tokens)?)),
            Token::Tag(name) => Ok(Requirement::Tag(name)),
            Token::Not => Ok(Requirement::Not(Box::new(parse_requirement(tokens)?))),
            tok @ (Token::LBracket | Token::RBracket) => Err(ParseError::UnexpectedToken(*tok)),
        },
        None => Err(ParseError::UnexpectedEnd),
    }
}

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
        match tokens.peek() {
            Some(Token::RBracket) => {
                // Consume the bracket
                tokens.next().unwrap();
                return Ok(reqs);
            }
            Some(_) => reqs.push(parse_requirement(tokens)?),
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
    // !foo
    assert_eq!(
        parse(&[Token::Not, Token::Tag("foo")]).as_deref(),
        Ok(&[Requirement::Not(Box::new(Requirement::Tag("foo")))][..])
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
    // @any[a !@all[b !c !@foo[d e]]] foo-tag
    assert_eq!(
        parse(&[
            Token::FunIdent("any"),
            Token::LBracket,
            Token::Tag("a"),
            Token::Not,
            Token::FunIdent("all"),
            Token::LBracket,
            Token::Tag("b"),
            Token::Not,
            Token::Tag("c"),
            Token::Not,
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
                    Requirement::Not(Box::new(Requirement::FnCall(FnCall {
                        name: "all",
                        params: vec![
                            Requirement::Tag("b"),
                            Requirement::Not(Box::new(Requirement::Tag("c"))),
                            Requirement::Not(Box::new(Requirement::FnCall(FnCall {
                                name: "foo",
                                params: vec![Requirement::Tag("d"), Requirement::Tag("e"),]
                            })))
                        ]
                    })))
                ]
            }),
            Requirement::Tag("foo-tag")
        ][..])
    );
}
