use crate::cpu::{jump_location::JumpLocation, main::CPU, CPUType};

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
    JumpLocation(JumpLocation),
    FunctionName,
    Bracket,
    Keyword,
    String,
    Number(CPUType),
    Comment,
    Comma,
    NewLine,
}

#[derive(Debug)]
pub enum LexerError {
    NoFunctionNameGiven,
    UnexpectedInstruction,
    ExpectedFunctionName,
    TokenListIsEmpty,
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
            tokens: Vec::new(),
            strings: Vec::new(),
            line_number: 0,
        }
    }
    pub fn run(&mut self, line: String) -> Result<(), LexerError> {
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
                    });
                }
                "call" => {
                    self.tokens.push(Token {
                        token_type: TokenType::OpCode,
                        value: str.to_string(),
                    });
                    if !string_iter.peek().is_some() {
                        return Err(LexerError::ExpectedFunctionName);
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
                    if string_iter.peek().unwrap() == &"{" {
                        return Err(LexerError::NoFunctionNameGiven);
                    }
                    self.tokens.push(Token {
                        token_type: TokenType::FunctionName,
                        value: string_iter.next().unwrap().to_string(),
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
                    });
                }
                "A" => {
                    self.tokens.push(Token {
                        token_type: TokenType::Accumulator,
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
                    } else if str.chars().nth(0) == Some('#') {
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
                                line: self.line_number,
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
                                println!("\n--------\n{}\n---------\n", str);
                                return Err(LexerError::UnexpectedInstruction);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
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

    pub fn get_tokens(self) -> Result<Vec<Token>, LexerError> {
        if self.tokens.is_empty() {
            Err(LexerError::TokenListIsEmpty)
        } else {
            Ok(self.tokens)
        }
    }

    pub fn show_tokens(self) {
        self.tokens.iter().for_each(|token| {
            println!("Token {{");
            println!("  TokenType: {:?}", token.token_type);
            println!("  Value    : {:?}", token.value);
            println!("}}")
        })
    }
}
