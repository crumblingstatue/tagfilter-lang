use super::*;

#[test]
fn simple() {
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
}

#[test]
fn complex() {
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
