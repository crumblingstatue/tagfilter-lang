#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Token<'a> {
    /// Function identifier e.g. `@any`.
    ///
    /// The payload is the identifier without the `@` part.
    FunIdent(&'a str),
    /// A tag, e.g. `forest` or `cool-stuff`
    Tag(&'a str),
    /// `[`
    LBracket,
    /// `]`
    RBracket,
}

enum Status {
    Init,
    FunIdent,
    Tag,
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
                c if c.is_ascii_whitespace() => {}
                _ => {
                    begin = pos;
                    status = Status::Tag;
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
                    };
                }
            }
            Status::Tag => {
                if byte.is_ascii_whitespace() || matches!(byte, b'[' | b']') {
                    tokens.push(Token::Tag(&input[begin..pos]));
                    status = Status::Init;
                    match byte {
                        b'[' => tokens.push(Token::LBracket),
                        b']' => tokens.push(Token::RBracket),
                        _ => {}
                    };
                }
            }
        }
    }
    // End of stream handling
    match status {
        Status::Init => {}
        Status::FunIdent => tokens.push(Token::FunIdent(&input[begin..])),
        Status::Tag => {
            tokens.push(Token::Tag(&input[begin..]));
        }
    }
    tokens
}

#[test]
fn test_tokenize() {
    assert_eq!(&tokenize("foo"), &[Token::Tag("foo")]);
    assert_eq!(
        &tokenize("foo bar baz"),
        &[Token::Tag("foo"), Token::Tag("bar"), Token::Tag("baz")]
    );
    assert_eq!(&tokenize("@any"), &[Token::FunIdent("any")]);
    assert_eq!(
        &tokenize("@all[foo bar]"),
        &[
            Token::FunIdent("all"),
            Token::LBracket,
            Token::Tag("foo"),
            Token::Tag("bar"),
            Token::RBracket
        ]
    );
    assert_eq!(
        &tokenize("@any[foo @all[bar baz] @special] ["),
        &[
            Token::FunIdent("any"),
            Token::LBracket,
            Token::Tag("foo"),
            Token::FunIdent("all"),
            Token::LBracket,
            Token::Tag("bar"),
            Token::Tag("baz"),
            Token::RBracket,
            Token::FunIdent("special"),
            Token::RBracket,
            Token::LBracket,
        ]
    );
}
