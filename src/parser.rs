use std::iter::Peekable;

use thiserror::Error;

use crate::tokenizer::Token;

#[cfg(test)]
mod tests;

/// Error that can happen when parsing a requirements string
#[derive(Debug, Error, PartialEq)]
pub enum ParseError<'a> {
    /// An unexpected token was encountered
    #[error("Unexpected token {0:?}")]
    UnexpectedToken(Token<'a>),
    /// Expected a specific token, but got another one (or none)
    #[error("Mismatch: expected {expected:?} got {got:?}")]
    Mismatch {
        /// The token we expected
        expected: Token<'a>,
        /// The token we actually got
        got: Option<Token<'a>>,
    },
    /// Unexpected end when we were expecting more tokens
    #[error("Unexpected end")]
    UnexpectedEnd,
}

/// A requirement for how to match tags
#[derive(Debug, PartialEq)]
pub enum Requirement<'a> {
    /// Tag requirement like `foo`
    Tag(&'a str),
    /// `$mytag` should be matched exactly (no implies-relationship)
    TagExact(&'a str),
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
            Token::TagExact(name) => Ok(Requirement::TagExact(name)),
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
