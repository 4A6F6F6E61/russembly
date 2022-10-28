#![allow(dead_code)]

use crate::{
    cpu::{lexer_error, printx, CPUType, JumpLocation, PrintT},
    log,
};
#[cfg(not(target_arch = "wasm32"))]
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::fmt::Write;

#[derive(Clone, Debug)]
pub struct Line {
    pub tokens: Vec<Token>,
    pub as_string: String,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<Token>,
    pub lines: Vec<Line>,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

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
    Generic,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug)]
pub struct Lexer {
    lines: Vec<Line>,
    functions: Vec<Function>,
    tokens: Vec<Token>,
    strings: Vec<String>,
    progress_bar: ProgressBar,
    pub syntax: HashMap<&'static str, &'static str>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug)]
pub struct Lexer {
    lines: Vec<Line>,
    functions: Vec<Function>,
    tokens: Vec<Token>,
    strings: Vec<String>,
    pub syntax: HashMap<&'static str, &'static str>,
}
impl Lexer {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Lexer {
        Lexer {
            lines: vec![],
            functions: vec![],
            tokens: vec![],
            strings: vec![],
            progress_bar: ProgressBar::new(10000), // For two decimal
            syntax: HashMap::from([("function", "def"), ("", "")]),
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Lexer {
        Lexer {
            lines: vec![],
            functions: vec![],
            tokens: vec![],
            strings: vec![],
            syntax: HashMap::from([("function", "def"), ("", "")]),
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn setup_pb(&mut self) {
        self.progress_bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}% ({eta})",
            )
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
            })
            .progress_chars("#-"),
        );
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn finish_pb(&mut self) {
        self.progress_bar
            .finish_with_message("Finished parsing tokens");
    }

    pub fn run(&mut self, line: String, max_lines: usize) {
        self.tokens = vec![];
        if max_lines == 0 {
            log!(Error, "division by zero");
            lexer_error();
        }
        let percent: f32 = (self.line_number() as f32) / max_lines as f32 * 100.0;
        self.generate_strings(line.clone());
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
                        } /* Why would u need the somments in the Tokentree???
                          self.tokens.push(Token {
                              token_type: TokenType::Comment,
                              value: comment,
                          }); */
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
                                if !self.tokens.is_empty() {
                                    self.tokens.push(Token {
                                        token_type: TokenType::Generic,
                                        value: str.to_string(),
                                    });
                                } else {
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
        }
        if cfg!(target_arch = "wasm32") {
            log!(Info, f("Parsing lines {:.2}%", percent));
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.progress_bar.set_position((percent as u64) * 100);
        }
        self.lines.push(Line {
            tokens: self.tokens.clone(),
            as_string: line,
        });
    }

    pub fn generate_functions(&mut self) /*-> Option<Vec<Function>>*/
    {
        let mut function_name = "";
        let mut function_arguments: Vec<Token> = vec![];
        let mut function_body: Vec<Line> = vec![];

        for line in &self.lines {
            if check_line_for_fn(line.clone()) {
                if function_name != "" {
                    self.functions.push(Function {
                        name: function_name.to_string(),
                        arguments: function_arguments.clone(),
                        lines: function_body.clone(),
                    });
                    function_name = "";
                    function_arguments = vec![];
                    function_body = vec![];
                }
                let mut tokens = line.tokens.iter().peekable();
                while tokens.peek().is_some() {
                    let token = tokens.next().unwrap();
                    match &token.token_type {
                        TokenType::Comment => {
                            continue;
                        }
                        TokenType::Keyword => match token.value.as_str() {
                            "fn" => {
                                let syntax_fn = || {
                                    log!(Syntax, "\nfn `name` (`arguments`) {\n`code`\n}");
                                };
                                if let (Some(fn_name), Some(open_function_bracket)) =
                                    (tokens.next(), tokens.next())
                                {
                                    if open_function_bracket.value != "(" {
                                        let bracket = &open_function_bracket.value;
                                        log!(Error, f("Expected `(` but found {bracket}"));
                                        syntax_fn();
                                        break;
                                    }
                                    function_name = &fn_name.value;
                                    // generate function arguments
                                    while tokens.peek().is_some() {
                                        let temp = tokens.next().unwrap();
                                        match temp.token_type {
                                            TokenType::Bracket => {
                                                if temp.value != ")" {
                                                    log!(Error, "Expected closing breackets");
                                                    syntax_fn();
                                                    break;
                                                } else {
                                                    break;
                                                }
                                            }
                                            _ => {
                                                function_arguments.push(temp.clone());
                                            }
                                        }
                                    }
                                } else {
                                    log!(Error, "Expected function name and opening brackets");
                                    syntax_fn();
                                    break;
                                }
                            }
                            _ => { /* already checked */ }
                        },
                        TokenType::Bracket => {}
                        _ => {
                            log!(Error, "Top level code is not allowed");
                            println!("{:#?}", token)
                        }
                    }
                }
            } else {
                function_body.push(line.clone());
            }
        }
        if function_name != "" {
            self.functions.push(Function {
                name: function_name.to_string(),
                arguments: function_arguments.clone(),
                lines: function_body.clone(),
            })
        }
        //Some(self.functions.clone())
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

    pub fn get_functions(&self) -> Option<Vec<Function>> {
        if self.functions.is_empty() {
            log!(Error, "No functions found (empty)");
            return None;
        }
        Some(self.functions.clone())
    }

    pub fn get_tokens(self) -> Option<Vec<Token>> {
        if self.tokens.is_empty() {
            log!(Error, "No tokens found (empty)");
            return None;
        }
        Some(self.tokens.clone())
    }

    pub fn get_lines(self) -> Option<Vec<Line>> {
        if self.lines.is_empty() {
            log!(Error, "No lines found (empty)");
            return None;
        }
        Some(self.lines.clone())
    }

    pub fn show_tokens(self) {
        self.tokens.iter().for_each(|token| {
            println!("Token {{");
            println!("  TokenType: {:?}", token.token_type);
            println!("  Value    : {:?}", token.value);
            println!("}}")
        })
    }

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

fn check_line_for_fn(line: Line) -> bool {
    if line.tokens.len() != 0 {
        if line.tokens[0].value == "fn" {
            return true;
        }
    }
    false
}
