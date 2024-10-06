use std::env;
use std::fs;
use std::process::exit;
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
    for (line, token) in file_contents.split(" ").enumerate() {
        for ch in token.chars() {
            match ch {
                ';' => {
                    println!("SEMICOLON {ch} null");
                    continue;
                }
                '-' => {
                    println!("MINUS {ch} null");
                    continue;
                }
                '+' => {
                    println!("PLUS {ch} null");
                    continue;
                }
                ',' => {
                    println!("COMMA {ch} null");
                    continue;
                }
                '.' => {
                    println!("DOT {ch} null");
                    continue;
                }
                '*' => {
                    println!("STAR {ch} null");
                    continue;
                }
                '{' => {
                    println!("LEFT_BRACE {ch} null");
                    continue;
                }
                '}' => {
                    println!("RIGHT_BRACE {ch} null");
                    continue;
                }
                '(' => {
                    println!("LEFT_PAREN {ch} null");
                    continue;
                }
                ')' => {
                    println!("RIGHT_PAREN {ch} null");
                    continue;
                }
                _ => {
                    eprintln!("[line {}] Error: Unexpected character: {ch}", line + 1);
                    exit_code = 65
                }
            }
        }
        println!("EOF  null");
    }
    exit(exit_code);
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
