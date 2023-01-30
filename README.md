```sh
[printvars-core/src/lib.rs:15] &args = TokenStream [
    Ident {
        ident: "p",
        span: #0 bytes(48..49),
    },
    Punct {
        ch: ',',
        spacing: Alone,
        span: #0 bytes(49..50),
    },
    Ident {
        ident: "n",
        span: #0 bytes(51..52),
    },
]
[printvars-core/src/lib.rs:16] &input = TokenStream [
    Ident {
        ident: "fn",
        span: #0 bytes(55..57),
    },
    Ident {
        ident: "factorial",
        span: #0 bytes(58..67),
    },
    Group {
        delimiter: Parenthesis,
        stream: TokenStream [
            Ident {
                ident: "mut",
                span: #0 bytes(68..71),
            },
            Ident {
                ident: "n",
                span: #0 bytes(72..73),
            },
            Punct {
                ch: ':',
                spacing: Alone,
                span: #0 bytes(73..74),
            },
            Ident {
                ident: "u64",
                span: #0 bytes(75..78),
            },
        ],
        span: #0 bytes(67..79),
    },
    Punct {
        ch: '-',
        spacing: Joint,
        span: #0 bytes(80..81),
    },
    Punct {
        ch: '>',
        spacing: Alone,
        span: #0 bytes(81..82),
    },
    Ident {
        ident: "u64",
        span: #0 bytes(83..86),
    },
    Group {
        delimiter: Brace,
        stream: TokenStream [
            Ident {
                ident: "let",
                span: #0 bytes(93..96),
            },
            Ident {
                ident: "mut",
                span: #0 bytes(97..100),
            },
            Ident {
                ident: "p",
                span: #0 bytes(101..102),
            },
            Punct {
                ch: '=',
                spacing: Alone,
                span: #0 bytes(103..104),
            },
            Literal {
                kind: Integer,
                symbol: "1",
                suffix: None,
                span: #0 bytes(105..106),
            },
            Punct {
                ch: ';',
                spacing: Alone,
                span: #0 bytes(106..107),
            },
            Ident {
                ident: "while",
                span: #0 bytes(112..117),
            },
            Ident {
                ident: "n",
                span: #0 bytes(118..119),
            },
            Punct {
                ch: '>',
                spacing: Alone,
                span: #0 bytes(120..121),
            },
            Literal {
                kind: Integer,
                symbol: "1",
                suffix: None,
                span: #0 bytes(122..123),
            },
            Group {
                delimiter: Brace,
                stream: TokenStream [
                    Ident {
                        ident: "p",
                        span: #0 bytes(134..135),
                    },
                    Punct {
                        ch: '*',
                        spacing: Joint,
                        span: #0 bytes(136..137),
                    },
                    Punct {
                        ch: '=',
                        spacing: Alone,
                        span: #0 bytes(137..138),
                    },
                    Ident {
                        ident: "n",
                        span: #0 bytes(139..140),
                    },
                    Punct {
                        ch: ';',
                        spacing: Alone,
                        span: #0 bytes(140..141),
                    },
                    Ident {
                        ident: "n",
                        span: #0 bytes(150..151),
                    },
                    Punct {
                        ch: '-',
                        spacing: Joint,
                        span: #0 bytes(152..153),
                    },
                    Punct {
                        ch: '=',
                        spacing: Alone,
                        span: #0 bytes(153..154),
                    },
                    Literal {
                        kind: Integer,
                        symbol: "1",
                        suffix: None,
                        span: #0 bytes(155..156),
                    },
                    Punct {
                        ch: ';',
                        spacing: Alone,
                        span: #0 bytes(156..157),
                    },
                ],
                span: #0 bytes(124..163),
            },
            Ident {
                ident: "p",
                span: #0 bytes(168..169),
            },
        ],
        span: #0 bytes(87..171),
    },
]
```
