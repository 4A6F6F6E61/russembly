use {
    crate::{
        cpu::{jump_location::JumpLocation, printx, CPUType, PrintT},
        log,
    },
    std::process::exit,
};

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    OpCode,
    Accumulator,
    Port,
    JumpLocation(JumpLocation),
    FunctionName,
    VarName,
    Bracket,
    Keyword,
    String,
    Number(CPUType),
    Comment,
    Comma,
    NewLine,
}

#[derive(Clone, Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
    strings: Vec<String>,
    line_number: usize,
}
impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            tokens: vec![],
            strings: vec![],
            line_number: 0,
        }
    }
    pub fn run(&mut self, line: String, max_lines: usize) -> usize {
        let mut errors = 0usize;
        let ln = self.line_number;

        if max_lines == 0 {
            log!(Error, "division by zero");
        }
        let percent: f32 = (ln as f32 + 1.0) / max_lines as f32 * 100.0;
        self.line_number += 1;
        self.generate_strings(line);
        let mut string_iter = self.strings.iter().peekable();
        while string_iter.peek().is_some() {
            let str = string_iter.next().unwrap();
            match str.as_str() {
                "push" | "pop" | "mov" | "add" | "sub" | "mul" | "div" | "adds" | "subs"
                | "muls" | "divs" | "djnzs" | "jmp" | "setb" | "end" | "prnt" => {
                    self.tokens.push(Token {
                        token_type: TokenType::OpCode,
                        value: str.to_string(),
                        line: self.line_number,
                    });
                }
                "call" => {
                    self.tokens.push(Token {
                        token_type: TokenType::OpCode,
                        value: str.to_string(),
                        line: self.line_number,
                    });
                    if !string_iter.peek().is_some() {
                        printx(
                            PrintT::Error,
                            format!("Expected Function name at line {}", self.line_number).as_str(),
                        );
                        errors += 1;
                    }
                    self.tokens.push(Token {
                        token_type: TokenType::FunctionName,
                        value: string_iter.next().unwrap().to_string(),
                        line: self.line_number,
                    });
                }
                "{" | "}" | "[" | "]" | "(" | ")" => {
                    self.tokens.push(Token {
                        token_type: TokenType::Bracket,
                        value: str.to_string(),
                        line: self.line_number,
                    });
                }
                "fn" => {
                    self.tokens.push(Token {
                        token_type: TokenType::Keyword,
                        value: str.to_string(),
                        line: self.line_number,
                    });
                    if string_iter.peek().unwrap() == &"{" {
                        printx(
                            PrintT::Error,
                            format!("Expected Function name at line {}", self.line_number).as_str(),
                        );
                    }
                    self.tokens.push(Token {
                        token_type: TokenType::FunctionName,
                        value: string_iter.next().unwrap().to_string(),
                        line: self.line_number,
                    });
                }
                "let" => {
                    self.tokens.push(Token {
                        token_type: TokenType::Keyword,
                        value: str.to_string(),
                        line: self.line_number,
                    });
                    self.tokens.push(Token {
                        token_type: TokenType::VarName,
                        value: string_iter.next().unwrap().to_string(),
                        line: self.line_number,
                    });
                }
                "\"" => {
                    let mut string = String::new();
                    while string_iter.peek().is_some() {
                        let str = string_iter.next().unwrap();
                        if str == "\"" {
                            break;
                        }
                        string.push_str(str);
                    }
                    string = string.replace("\\n", "\n");
                    self.tokens.push(Token {
                        token_type: TokenType::String,
                        value: string,
                        line: self.line_number,
                    });
                }
                "A" => {
                    self.tokens.push(Token {
                        token_type: TokenType::Accumulator,
                        value: str.to_string(),
                        line: self.line_number,
                    });
                }
                "," => {
                    self.tokens.push(Token {
                        token_type: TokenType::Comma,
                        value: str.to_string(),
                        line: self.line_number,
                    });
                }
                "nl" => {
                    self.tokens.push(Token {
                        token_type: TokenType::NewLine,
                        value: "\n".to_string(),
                        line: self.line_number,
                    });
                }
                _ => {
                    if str.starts_with("P") {
                        self.tokens.push(Token {
                            token_type: TokenType::Port,
                            value: str.to_string(),
                            line: self.line_number,
                        });
                    } else if str.chars().nth(0) == Some('#') {
                        let mut comment = String::new();
                        while string_iter.peek().is_some() {
                            comment.push_str(string_iter.next().unwrap());
                            comment.push_str(" ");
                        }
                        self.tokens.push(Token {
                            token_type: TokenType::Comment,
                            value: comment,
                            line: self.line_number,
                        });
                        continue;
                    } else if str.ends_with(":") {
                        let mut tmp = str.chars();
                        tmp.next_back();
                        self.tokens.push(Token {
                            token_type: TokenType::JumpLocation(JumpLocation {
                                name: tmp.as_str().to_string(),
                                line: self.line_number,
                            }),
                            value: str.to_string(),
                            line: self.line_number,
                        });
                    } else {
                        match str.parse::<CPUType>() {
                            Ok(x) => {
                                self.tokens.push(Token {
                                    token_type: TokenType::Number(x),
                                    value: str.to_string(),
                                    line: self.line_number,
                                });
                            }
                            Err(_) => {
                                errors += 1;
                                println!("\n--------\n{}\n---------\n", str);
                                let ln = self.line_number;
                                log!(Error, f("Unexpected instruction at line {}", ln));
                            }
                        }
                    }
                }
            }
        }
        log!(Info, f("Parsing tokens {:.2}%", percent));
        errors
    }
    fn generate_strings(&mut self, line: String) {
        self.strings.clear();
        let mut temp_string = String::new();
        for char in line.chars() {
            match char {
                ' ' => {
                    if temp_string.len() > 0 {
                        self.strings.push(temp_string);
                        temp_string = String::new();
                    }
                }
                '(' | ')' | '{' | '}' | '[' | ']' | ',' | '=' | '+' | '-' | '*' | '"' => {
                    if temp_string.len() > 0 {
                        self.strings.push(temp_string);
                        temp_string = String::new();
                    }
                    self.strings.push(char.to_string());
                }
                _ => temp_string.push(char),
            }
        }
        if temp_string.len() > 0 {
            self.strings.push(temp_string);
        }
    }

    pub fn get_tokens(self) -> Option<Vec<Token>> {
        if self.tokens.is_empty() {
            log!(Error, "No tokens found is empty");
            return None;
        }
        Some(self.tokens)
    }

    #[allow(dead_code)]
    pub fn show_tokens(self) {
        self.tokens.iter().for_each(|token| {
            println!("Token {{");
            println!("  TokenType: {:?}", token.token_type);
            println!("  Value    : {:?}", token.value);
            println!("}}")
        })
    }
}
