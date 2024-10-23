use core::fmt;
use std::process::exit;

#[derive(Debug)]
pub struct Lox {
    had_error: bool,
    output: Vec<Token>,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            had_error: false,
            output: Vec::new(),
        }
    }
    pub fn add(&mut self, str: Token) {
        self.output.push(str);
    }

    pub fn print_to_stdout(&self) {
        for tkn in self.output.clone() {
            println!("{:#?}", tkn);
        }
        exit(if !self.had_error { 0 } else { 65 })
    }
}
pub fn tokenize(file_contents: String) {
    let mut exit_code = 0;
    let mut lox = Lox::new();
    let mut last_line = 0;
    for (line, token) in file_contents.lines().enumerate() {
        last_line = line;
        let mut chariter = token.chars().peekable();
        while let Some(ch) = chariter.next() {
            match ch {
                '\"' => {
                    let mut string = String::new();
                    let mut is_terminated = false;
                    for ch in chariter.by_ref() {
                        if ch == '\"' {
                            is_terminated = true;
                            break;
                        }
                        string.push(ch);
                    }
                    if !is_terminated {
                        eprintln!("[line {}] Error: Unterminated string.", line + 1);
                        lox.had_error = true;
                        exit_code = 65;
                    } else {
                        // lox.add(format!("STRING \"{string}\" {string}"));
                        lox.add(Token::new(
                            TokenType::STRING,
                            format!("\"{}\"", string),
                            Some(Literal::String(string)),
                            line + 1,
                        ));
                    }
                }
                '/' => {
                    if let Some(after_slash) = chariter.peek() {
                        if *after_slash == '/' {
                            for ch in chariter.by_ref() {
                                if ch == '\n' {
                                    break;
                                }
                            }
                        }
                    } else {
                        // lox.add(format!("SLASH {ch} null"));
                        lox.add(Token::new(TokenType::SLASH, ch.to_string(), None, line + 1));
                    }
                }
                ';' => lox.add(Token::new(
                    TokenType::SEMICOLON,
                    ch.to_string(),
                    None,
                    line + 1,
                )),
                '-' => lox.add(Token::new(TokenType::MINUS, ch.to_string(), None, line + 1)),
                '+' => lox.add(Token::new(TokenType::PLUS, ch.to_string(), None, line + 1)),
                ',' => lox.add(Token::new(TokenType::COMMA, ch.to_string(), None, line + 1)),
                '.' => lox.add(Token::new(TokenType::DOT, ch.to_string(), None, line + 1)),
                '*' => lox.add(Token::new(TokenType::STAR, ch.to_string(), None, line + 1)),
                '{' => lox.add(Token::new(
                    TokenType::LEFT_BRACE,
                    ch.to_string(),
                    None,
                    line + 1,
                )),
                '}' => lox.add(Token::new(
                    TokenType::RIGHT_BRACE,
                    ch.to_string(),
                    None,
                    line + 1,
                )),
                '(' => lox.add(Token::new(
                    TokenType::LEFT_PAREN,
                    ch.to_string(),
                    None,
                    line + 1,
                )),
                ')' => lox.add(Token::new(
                    TokenType::RIGHT_PAREN,
                    ch.to_string(),
                    None,
                    line + 1,
                )),
                '!' => {
                    if let Some(after_bang) = chariter.peek() {
                        if *after_bang == '=' {
                            lox.add(Token::new(
                                TokenType::BANG_EQUAL,
                                "!=".to_string(),
                                None,
                                line + 1,
                            ));
                            chariter.next();
                        } else {
                            // lox.add(format!("BANG {ch} null"));
                            lox.add(Token::new(TokenType::BANG, ch.to_string(), None, line + 1));
                        }
                    }
                }
                '>' => {
                    if let Some(after_less) = chariter.peek() {
                        let (did_match, log) =
                            make_next(TokenType::GREATER, ch, *after_less, line + 1);
                        if did_match {
                            chariter.next();
                        }
                        lox.add(log);
                    } else {
                        // lox.add("GREATER > null".to_string());
                        lox.add(Token::new(
                            TokenType::GREATER,
                            ch.to_string(),
                            None,
                            line + 1,
                        ));
                    }
                }
                '<' => {
                    if let Some(after_less) = chariter.peek() {
                        let (did_match, log) =
                            make_next(TokenType::LESS, ch, *after_less, line + 1);
                        if did_match {
                            chariter.next();
                        }
                        lox.add(log);
                    } else {
                        // lox.add("LESS < null".to_string());
                        lox.add(Token::new(TokenType::LESS, ch.to_string(), None, line + 1));
                    }
                }
                '=' => {
                    if let Some(after_equal) = chariter.peek() {
                        let (did_match, token) =
                            make_next(TokenType::EQUAL, ch, *after_equal, line + 1);
                        if did_match {
                            chariter.next();
                        }
                        lox.add(token);
                    } else {
                        // lox.add("EQUAL = null".to_string());
                        lox.add(Token::new(TokenType::EQUAL, ch.to_string(), None, line + 1));
                    }
                }
                ' ' | '\r' | '\t' | '\n' => {}
                n => {
                    if n.is_alphabetic() || n == '_' {
                        let mut accumilate = n.to_string();
                        while let Some(next) = chariter.peek() {
                            if next.is_alphanumeric() || *next == '_' && *next != ' ' {
                                accumilate.push(*next);
                                chariter.next();
                            } else {
                                break;
                            }
                        }

                        let keyword = match accumilate.as_str() {
                            "and" => TokenType::AND,
                            "class" => TokenType::CLASS,
                            "else" => TokenType::ELSE,
                            "false" => TokenType::FALSE,
                            "for" => TokenType::FOR,
                            "fun" => TokenType::FUN,
                            "if" => TokenType::IF,
                            "nil" => TokenType::NIL,
                            "or" => TokenType::OR,
                            "print" => TokenType::PRINT,
                            "return" => TokenType::RETURN,
                            "super" => TokenType::SUPER,
                            "this" => TokenType::THIS,
                            "true" => TokenType::TRUE,
                            "var" => TokenType::VAR,
                            "while" => TokenType::WHILE,
                            _ => TokenType::IDENTIFIER,
                        };

                        // lox.add(format!("IDENTIFIER {accumilate} null"));
                        lox.add(Token::new(keyword, accumilate, None, line + 1));
                        continue;
                    } else if n.is_numeric() {
                        let mut accumilate = n.to_string();
                        while let Some(next) = chariter.peek() {
                            if next.is_numeric() || *next == '.' {
                                accumilate.push(*next);
                                chariter.next();
                            } else {
                                break;
                            }
                        }
                        let current_number: f64 = accumilate.parse().unwrap();

                        // lox.add(format!("NUMBER {accumilate} {:#?}", current_number));
                        lox.add(Token::new(
                            TokenType::NUMBER,
                            accumilate,
                            Some(Literal::Number(current_number)),
                            line + 1,
                        ));
                        continue;
                    }

                    eprintln!("[line {}] Error: Unexpected character: {ch}", line + 1);
                    lox.had_error = true;
                    exit_code = 65
                }
            }
        }
    }
    lox.add(Token::new(
        TokenType::EOF,
        "EOF".to_string(),
        None,
        last_line + 1,
    ));
    lox.print_to_stdout();
    exit(exit_code);
}

pub fn make_next(
    current_token: TokenType,
    current_char: char,
    next_symbol: char,
    line: usize,
) -> (bool, Token) {
    // println!("{arg}");
    if next_symbol == '=' {
        let next_token_type = match current_token {
            TokenType::LESS => TokenType::LESS_EQUAL,
            TokenType::GREATER => TokenType::GREATER_EQUAL,
            TokenType::EQUAL => TokenType::EQUAL_EQUAL,
            rest => {
                panic!("Not Valid Token `{:#?}`", rest);
            }
        };

        return (
            true,
            Token::new(
                next_token_type,
                format!("{}{}", current_char, next_symbol),
                None,
                line,
            ),
        );

        // return (true, lex_log.to_string());
    }
    (
        false,
        Token::new(current_token, current_char.to_string(), None, line),
    )
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Bool(bool),
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.token_type == TokenType::EOF {
            write!(f, "{:#?}  null", self.token_type)
        } else {
            write!(
                f,
                "{:?} {} {}",
                self.token_type,
                self.lexeme,
                if let Some(value) = &self.literal {
                    match value {
                        Literal::String(s) => s.to_string(),
                        Literal::Number(n) => {
                            // If it's a whole number, show one decimal place
                            if n.fract() == 0.0 {
                                format!("{:.1}", n)
                            } else {
                                // Otherwise, use the number's natural precision
                                n.to_string()
                            }
                        }
                        Literal::Bool(b) => b.to_string(),
                    }
                } else {
                    "null".to_string()
                }
            )
        }
    }
}
