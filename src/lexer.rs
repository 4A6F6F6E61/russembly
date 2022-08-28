use conv::ValueFrom;

use crate::cpu::CPUType;

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum TokenType {
    OpCode,
    Accumulator,
    Port,
    JumpLocation,
    FunctionName,
    Bracket,
    Keyword,
    String,
    Number,
    Comment,
    Comma,
}

pub enum LexerError {
    NoFunctionNameGiven,
    UnexpectedInstruction,
}

#[derive(Clone, Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
    strings: Vec<String>,
}
impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            tokens: Vec::new(),
            strings: Vec::new(),
        }
    }
    pub fn run(&mut self, line: String) -> Result<(), LexerError> {
        self.generate_strings(line);
        let mut string_iter = self.strings.iter().peekable();
        while string_iter.peek().is_some() {
            let str = string_iter.next().unwrap();
            match str.as_str() {
                "push" | "mov" | "add" | "sub" | "mul" | "div" | "djnz" | "jmp" | "setb"
                | "end" => {
                    self.tokens.push(Token {
                        token_type: TokenType::OpCode,
                        value: str.to_string(),
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
                    } else {
                        match str.parse::<CPUType>() {
                            Ok(_) => {
                                self.tokens.push(Token {
                                    token_type: TokenType::Number,
                                    value: str.to_string(),
                                });
                            }
                            Err(_) => {
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
}
