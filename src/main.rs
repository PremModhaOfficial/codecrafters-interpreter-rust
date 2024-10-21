use std::{env, fmt, fs, process::exit};

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
        for tkn in self.output.clone() {
            println!("{}", tkn);
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
    for (line, token) in file_contents.lines().enumerate() {
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
                        lox.log(format!("STRING \"{string}\" {string}"));
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
                        lox.log(format!("SLASH {ch} null"));
                    }
                }
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
                '>' => {
                    if let Some(after_less) = chariter.peek() {
                        let (did_match, log) = make_next("GREATER", ch, *after_less);
                        if did_match {
                            chariter.next();
                        }
                        lox.log(log);
                    } else {
                        lox.log("GREATER > null".to_string());
                    }
                }
                '<' => {
                    if let Some(after_less) = chariter.peek() {
                        let (did_match, log) = make_next("LESS", ch, *after_less);
                        if did_match {
                            chariter.next();
                        }
                        lox.log(log);
                    } else {
                        lox.log("LESS < null".to_string());
                    }
                }
                '=' => {
                    if let Some(after_equal) = chariter.peek() {
                        let (did_match, log) = make_next("EQUAL", ch, *after_equal);
                        if did_match {
                            chariter.next();
                        }
                        lox.log(log);
                    } else {
                        lox.log("EQUAL = null".to_string());
                    }
                }
                ' ' | '\r' | '\t' | '\n' => {}
                n => lox.log("Number {n} {n}".to_string()),
                _ => {
                    eprintln!("[line {}] Error: Unexpected character: {ch}", line + 1);
                    lox.had_error = true;
                    exit_code = 65
                }
            }
        }
    }
    lox.log("EOF  null".to_string());
    lox.print_to_stdout();
    exit(exit_code);
}

fn make_next(current_str: &str, current_char: char, next_symbol: char) -> (bool, String) {
    // println!("{arg}");
    if next_symbol == '=' {
        let str = format!("{current_str}_EQUAL {current_char}{next_symbol} null");

        return (true, str.to_string());
    }
    (false, format!("{current_str} {current_char} null"))
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // ... (same as before, but without the associated values)
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

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.token_type,
            self.lexeme,
            if let Some(value) = &self.literal {
                match value {
                    Literal::String(s) => format!("\"{}\"", s),
                    Literal::Number(n) => n.to_string(),
                    Literal::Bool(b) => b.to_string(),
                }
            } else {
                "null".to_string()
            }
        )
    }
}
