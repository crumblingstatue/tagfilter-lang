#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Token<'a> {
    /// Function identifier e.g. `@any`.
    ///
    /// The payload is the identifier without the `@` part.
    FunIdent(&'a str),
    /// A tag, e.g. `forest` or `cool-stuff`
    Tag(&'a str),
    /// `$mytag` should be matched exactly (no implies-relationship)
    TagExact(&'a str),
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `!`
    Not,
}

enum Status {
    Init,
    FunIdent,
    Tag { exact: bool },
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut status = Status::Init;
    let mut tokens = Vec::new();
    let mut begin = 0;
    for (pos, byte) in input.bytes().enumerate() {
        match status {
            Status::Init => match byte {
                b'@' => {
                    begin = pos + 1;
                    status = Status::FunIdent;
                }
                b'[' => {
                    tokens.push(Token::LBracket);
                }
                b']' => {
                    tokens.push(Token::RBracket);
                }
                b'!' => {
                    tokens.push(Token::Not);
                }
                b'$' => {
                    begin = pos + 1;
                    status = Status::Tag { exact: true };
                }
                c if c.is_ascii_whitespace() => {}
                _ => {
                    begin = pos;
                    status = Status::Tag { exact: false };
                }
            },
            Status::FunIdent => {
                if matches!(byte, b'[' | b']') || byte.is_ascii_whitespace() {
                    tokens.push(Token::FunIdent(&input[begin..pos]));
                    status = Status::Init;
                    match byte {
                        b'[' => tokens.push(Token::LBracket),
                        b']' => tokens.push(Token::RBracket),
                        _ => {}
                    }
                }
            }
            Status::Tag { exact } => {
                if byte.is_ascii_whitespace() || matches!(byte, b'[' | b']') {
                    let tok = if exact {
                        Token::TagExact(&input[begin..pos])
                    } else {
                        Token::Tag(&input[begin..pos])
                    };
                    tokens.push(tok);
                    status = Status::Init;
                    match byte {
                        b'[' => tokens.push(Token::LBracket),
                        b']' => tokens.push(Token::RBracket),
                        _ => {}
                    }
                }
            }
        }
    }
    // End of stream handling
    match status {
        Status::Init => {}
        Status::FunIdent => tokens.push(Token::FunIdent(&input[begin..])),
        Status::Tag { exact } => {
            let tok = if exact {
                Token::TagExact(&input[begin..])
            } else {
                Token::Tag(&input[begin..])
            };
            tokens.push(tok);
        }
    }
    tokens
}
