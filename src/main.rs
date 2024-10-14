use std::char;
use std::env;
use std::fs;
use std::process::exit;

#[derive(Debug)]
struct Lox {
    had_error: bool,
    output: Vec<String>,
}

impl Lox {
    fn new() -> Self {
        Self {
            had_error: false,
            output: Vec::new(),
        }
    }
    fn log(&mut self, str: String) {
        self.output.push(str);
    }

    fn print_to_stdout(&self) {
        for line in self.output.clone() {
            println!("{line}");
        }
        exit(if !self.had_error { 0 } else { 65 })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            if !file_contents.is_empty() {
                tokenize(file_contents)
            } else {
                println!("EOF  null");
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}

fn tokenize(file_contents: String) {
    let mut exit_code = 0;
    let mut lox = Lox::new();
    for (line, token) in file_contents.split(" ").enumerate() {
        let mut chariter = token.chars().peekable();
        while let Some(ch) = chariter.next() {
            match ch {
                ';' => lox.log(format!("SEMICOLON {ch} null")),
                '-' => lox.log(format!("MINUS {ch} null")),
                '+' => lox.log(format!("PLUS {ch} null")),
                ',' => lox.log(format!("COMMA {ch} null")),
                '.' => lox.log(format!("DOT {ch} null")),
                '*' => lox.log(format!("STAR {ch} null")),
                '{' => lox.log(format!("LEFT_BRACE {ch} null")),
                '}' => lox.log(format!("RIGHT_BRACE {ch} null")),
                '(' => lox.log(format!("LEFT_PAREN {ch} null")),
                ')' => lox.log(format!("RIGHT_PAREN {ch} null")),
                '!' => {
                    if let Some(after_bang) = chariter.peek() {
                        if *after_bang == '=' {
                            lox.log("BANG_EQUAL != null".to_string());
                            chariter.next();
                        } else {
                            lox.log(format!("BANG {ch} null"));
                        }
                    }
                }
                '=' => {
                    if let Some(after_equal) = chariter.peek() {
                        let (did_match, log) = make_next("EQUAL", ch, *after_equal);
                        if did_match {
                            chariter.next();
                        }
                        lox.log(log);
                        // if *after_equal == '=' {
                        //     lox.log("EQUAL_EQUAL == null".to_string());
                        //     chariter.next();
                        // } else {
                        //     lox.log("EQUAL = null".to_string());
                        // }
                    }
                }
                _ => {
                    eprintln!("[line {}] Error: Unexpected character: {ch}", line + 1);
                    lox.had_error = true;
                    exit_code = 65
                }
            }
        }
        lox.log("EOF  null".to_string());
        lox.print_to_stdout();
    }
    exit(exit_code);
}

fn make_next(arg: &str, this: char, after_symbol: char) -> (bool, String) {
    println!("{arg}");
    if after_symbol == '=' {
        let str = format!("{arg}_EQUAL {this}{after_symbol} null");

        return (true, str.to_string());
    }
    return (false, "".to_string());
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
#[derive(std::fmt::Debug)]
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
