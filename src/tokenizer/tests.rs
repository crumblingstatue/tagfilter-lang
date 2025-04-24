use super::*;

#[test]
fn exact() {
    assert_eq!(&tokenize("$foo"), &[Token::TagExact("foo")]);
    assert_eq!(&tokenize("!$foo"), &[Token::Not, Token::TagExact("foo")]);
    assert_eq!(
        &tokenize("$foo $bar $baz"),
        &[
            Token::TagExact("foo"),
            Token::TagExact("bar"),
            Token::TagExact("baz")
        ]
    );
    assert_eq!(
        &tokenize("$foo bar $baz"),
        &[
            Token::TagExact("foo"),
            Token::Tag("bar"),
            Token::TagExact("baz")
        ]
    );
}

#[test]
fn tok() {
    assert_eq!(&tokenize("foo"), &[Token::Tag("foo")]);
    assert_eq!(&tokenize("!foo"), &[Token::Not, Token::Tag("foo")]);
    assert_eq!(
        &tokenize("foo bar baz"),
        &[Token::Tag("foo"), Token::Tag("bar"), Token::Tag("baz")]
    );
    assert_eq!(
        &tokenize("foo !bar baz"),
        &[
            Token::Tag("foo"),
            Token::Not,
            Token::Tag("bar"),
            Token::Tag("baz")
        ]
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
