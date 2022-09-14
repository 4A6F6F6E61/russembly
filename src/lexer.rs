use crate::{
    cpu::lexer_error,
    cpu::{jump_location::JumpLocation, CPUType},
    log,
};

#[derive(Clone, Debug)]
pub struct Line {
    pub tokens: Vec<Token>,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    OpCode,
    Accumulator,
    Port,
    Stack,
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
    lines: Vec<Line>,
    tokens: Vec<Token>,
    strings: Vec<String>,
}
impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            lines: vec![],
            tokens: vec![],
            strings: vec![],
        }
    }
    pub fn run(&mut self, line: String, max_lines: usize) {
        self.tokens = vec![];

        if max_lines == 0 {
            log!(Error, "division by zero");
            lexer_error();
        }
        let percent: f32 = (self.line_number() as f32) / max_lines as f32 * 100.0;
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
                    });
                }
                "call" => {
                    self.tokens.push(Token {
                        token_type: TokenType::OpCode,
                        value: str.to_string(),
                    });
                    if !string_iter.peek().is_some() {
                        printx(
                            PrintT::Error,
                            format!("Expected Function name at line {}", self.line_number())
                                .as_str(),
                        );
                        lexer_error();
                    }
                    self.tokens.push(Token {
                        token_type: TokenType::FunctionName,
                        value: string_iter.next().unwrap().to_string(),
                    });
                }
                "{" | "}" | "[" | "]" | "(" | ")" => {
                    self.tokens.push(Token {
                        token_type: TokenType::Bracket,
                        value: str.to_string(),
                    });
                }
                "fn" => {
                    self.tokens.push(Token {
                        token_type: TokenType::Keyword,
                        value: str.to_string(),
                    });
                    if string_iter.peek() == Some(&&"{".to_string()) {
                        let ln = self.line_number();
                        log!(Error, f("Expected Function name at line {}", ln));
                        lexer_error();
                    }
                    if let Some(nt) = string_iter.next() {
                        self.tokens.push(Token {
                            token_type: TokenType::FunctionName,
                            value: nt.to_string(),
                        });
                    }
                }
                "let" => {
                    self.tokens.push(Token {
                        token_type: TokenType::Keyword,
                        value: str.to_string(),
                    });
                    if let Some(ns) = string_iter.next() {
                        self.tokens.push(Token {
                            token_type: TokenType::VarName,
                            value: ns.to_string(),
                        });
                    }
                }
                "\"" => {
                    let mut string = String::new();
                    while string_iter.peek().is_some() {
                        if let Some(str) = string_iter.next() {
                            if str == "\"" {
                                break;
                            }
                            string.push_str(str);
                            string.push_str(" ");
                        }
                    }
                    string = string.replace("\\n", "\n");
                    self.tokens.push(Token {
                        token_type: TokenType::String,
                        value: string,
                    });
                }
                "A" => {
                    self.tokens.push(Token {
                        token_type: TokenType::Accumulator,
                        value: str.to_string(),
                    });
                }
                "Stack" => {
                    self.tokens.push(Token {
                        token_type: TokenType::Stack,
                        value: str.to_string(),
                    });
                }
                "," => {
                    self.tokens.push(Token {
                        token_type: TokenType::Comma,
                        value: str.to_string(),
                    });
                }
                "nl" => {
                    self.tokens.push(Token {
                        token_type: TokenType::NewLine,
                        value: "\n".to_string(),
                    });
                }
                _ => {
                    if str.starts_with("P") {
                        self.tokens.push(Token {
                            token_type: TokenType::Port,
                            value: str.to_string(),
                        });
                    } else if str.chars().nth(0) == Some(';') {
                        let mut comment = String::new();
                        while string_iter.peek().is_some() {
                            comment.push_str(string_iter.next().unwrap());
                            comment.push_str(" ");
                        }
                        self.tokens.push(Token {
                            token_type: TokenType::Comment,
                            value: comment,
                        });
                        continue;
                    } else if str.ends_with(":") {
                        let mut tmp = str.chars();
                        tmp.next_back();
                        self.tokens.push(Token {
                            token_type: TokenType::JumpLocation(JumpLocation {
                                name: tmp.as_str().to_string(),
                                line: self.line_number(),
                            }),
                            value: str.to_string(),
                        });
                    } else {
                        match str.parse::<CPUType>() {
                            Ok(x) => {
                                self.tokens.push(Token {
                                    token_type: TokenType::Number(x),
                                    value: str.to_string(),
                                });
                            }
                            Err(_) => {
                                log!(Lexer, f("\n---------\n{}\n---------", str));
                                let ln = self.line_number();
                                log!(Error, f("Unexpected instruction at line {}", ln));
                                lexer_error();
                            }
                        }
                    }
                }
            }
        }
        log!(Info, f("Parsing lines {:.2}%", percent));
        self.lines.push(Line {
            tokens: self.tokens.clone(),
        });
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

    #[allow(dead_code)]
    pub fn get_tokens(self) -> Option<Vec<Token>> {
        if self.tokens.is_empty() {
            log!(Error, "No tokens found is empty");
            return None;
        }
        Some(self.tokens)
    }

    pub fn get_lines(self) -> Option<Vec<Line>> {
        if self.lines.is_empty() {
            log!(Error, "No lines found (empty)");
            return None;
        }
        Some(self.lines)
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
    #[allow(dead_code)]
    pub fn show_lines(&self) {
        self.lines.iter().for_each(|lines| {
            println!("Line {{");
            lines.tokens.iter().for_each(|token| {
                println!("  Token {{");
                println!("    TokenType: {:?}", token.token_type);
                println!("    Value    : {:?}", token.value);
                println!("  }}");
            });
            println!("}}");
        });
    }
    pub fn line_number(&self) -> usize {
        self.lines.len() + 1
    }
}
