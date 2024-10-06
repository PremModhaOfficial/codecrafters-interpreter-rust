mod others {
    fn tokenize(file_contents: String) {
        for token in file_contents.split(" ") {
            println!("{token}");
        }
    }

    #[derive()]
    struct Token {
        token_type: TokenType,
        lexeme: String,
        litral: String,
        line: i32,
    }

    impl Token {
        fn new(token_type: TokenType, lexeme: String, litral: String, line: i32) -> Self {
            Self {
                token_type,
                lexeme,
                litral,
                line,
            }
        }
    }
    #[derive()]
    enum TokenType {
        // Single-character tokens.
        LeftParen,
        RightParen,
        LeftBrace,
        RightBrace,
        Comma,
        Dot,
        Minus,
        Plus,
        Semicolon,
        Slash,
        Star,

        // one or two character tokens.
        Bang,
        BangEqual,
        Equal,
        EqualEqual,
        Greater,
        GreaterEqual,
        Less,
        LessEqual,

        // literals.
        Identifier,
        String,
        Number,

        // keywords.
        And,
        Class,
        Else,
        False,
        Fun,
        For,
        If,
        Nil,
        Or,
        Print,
        Return,
        Super,
        This,
        True,
        Var,
        While,

        Eof,
    }
}
