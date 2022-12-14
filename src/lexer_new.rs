#![allow(dead_code)]
/* ### import logic for progress bar ###
#[cfg(not(target_arch = "wasm32"))]
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
#[cfg(not(target_arch = "wasm32"))]
use std::fmt::Write;*/
use {
    crate::{
        cpu::{lexer_error, printx, CPUType, PrintT},
        log,
    },
    serde::{Deserialize, Serialize},
    std::vec,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Token {
    Function(Function),
    LoopFunction(Function),
    Loop(Loop),
    Const(Let),
    Global(Let),
    Var(Let),
    Number(CPUType),
    String(String),
    OpCode(String),
    Port(String),
    Comment(String),
    If(If),
    End(i32),
    OpenSqBr(i32),
    CloseSqBr(i32),
    OpenRoBr(i32),
    CloseRoBr(i32),
    Stack,
    Accumulator,
    Comma,
    Generic,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Line {
    pub tokens: Vec<Token>,
    pub as_string: String,
}

// -----------------------------------------------------------------------
// Token structs
// -----------------------------------------------------------------------
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<String>,
    pub lines: Vec<Line>,
    pub tmp_lines: Vec<Vec<String>>,
    pub start_ln: i32,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Loop {
    pub lines: Vec<Line>,
    pub tmp_lines: Vec<Vec<String>>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Let {
    pub name: String,
    pub value: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct If {
    pub condition: String,
    pub id: i32,
}
// -----------------------------------------------------------------------
// Lexer structs
// -----------------------------------------------------------------------
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Lexer {
    pub tmp_ast: Vec<Token>,
    pub ast: Vec<Token>,
    strings: Vec<Vec<String>>,
    //progress_bar: ProgressBar,
    brackets: Brackets,
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Debug)]
pub struct Lexer {
    pub tmp_ast: Vec<Token>,
    pub ast: Vec<Token>,
    strings: Vec<Vec<String>>,
    brackets: Brackets,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Brackets {
    pub round: i32,
    pub square: i32,
    pub braces: i32,
}
// -----------------------------------------------------------------------
// Lexer implementation
// -----------------------------------------------------------------------
impl Lexer {
    pub fn new() -> Lexer {
        #[cfg(not(target_arch = "wasm32"))]
        return Lexer {
            tmp_ast: vec![],
            ast: vec![],
            strings: vec![],
            //progress_bar: ProgressBar::new(10000),
            brackets: Brackets {
                round: 0,
                square: 0,
                braces: 0,
            },
        };
        #[cfg(target_arch = "wasm32")]
        Lexer {
            tmp_ast: vec![],
            ast: vec![],
            strings: vec![],
            brackets: Brackets {
                round: 0,
                square: 0,
                braces: 0,
            },
        }
    }
    // --------------------------------
    // Progressbar setup
    // --------------------------------
    /*#[cfg(not(target_arch = "wasm32"))]
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
    }*/
    // --------------------------------
    // String vector generation
    // --------------------------------
    fn generate_strings(&mut self, code: String) {
        self.strings.clear();
        let line_count = code.lines().count();
        if line_count != 0 {
            code.lines().enumerate().for_each(|(i, line)| {
                let mut temp_string = String::new();
                self.strings.push(vec![]);
                for char in line.chars() {
                    match char {
                        ' ' => {
                            if temp_string.len() > 0 {
                                self.strings[i].push(temp_string);
                                temp_string = String::new();
                            }
                        }
                        '(' | ')' | '{' | '}' | '[' | ']' | ',' | '=' | '+' | '-' | '*' | '"'
                        | '#' => {
                            if temp_string.len() > 0 {
                                self.strings[i].push(temp_string);
                                temp_string = String::new();
                            }
                            self.strings[i].push(char.to_string());
                        }
                        _ => temp_string.push(char),
                    }
                }
                if temp_string.len() > 0 {
                    self.strings[i].push(temp_string);
                }
            });
        } else {
            log!(Lexer, "Please provide some code");
        }
    }
    // --------------------------------
    // Parsing
    // --------------------------------
    pub fn parse(&mut self, code: String) {
        self.top_level(code);

        for node in self.tmp_ast.clone() {
            match node {
                Token::Function(func) => {
                    let f = Token::Function(Function {
                        lines: self.low_level(func.tmp_lines.clone(), func.start_ln.clone()),
                        ..func
                    });
                    self.ast.push(f);
                }
                _ => {
                    self.ast.push(node.to_owned());
                }
            }
        }
        if self.brackets.braces > 0 {}
    }
    pub fn top_level(&mut self, code: String) {
        self.generate_strings(code); // generates a 2D string vector
                                     // ------------------------------
                                     // This is for toplevel only
                                     // ------------------------------
        let mut line_iter = self.strings.iter().peekable();
        let mut line_number = 0;
        while line_iter.peek().is_some() {
            line_number += 1;
            let next_line = line_iter.next().unwrap();
            let mut string_iter = next_line.iter().peekable();
            while string_iter.peek().is_some() {
                let str = string_iter.next().unwrap();
                match str.as_str() {
                    "fn" | "loop" => {
                        let mut loop_: bool = false;
                        if str.as_str() == "loop" {
                            loop_ = true;
                            string_iter.next();
                        }
                        let syntax_fn = || {
                            log!(Syntax, "\nfn `name` (`arguments`) {\n`code`\n}");
                        };
                        if let (Some(fn_name), Some(fn_op_br)) =
                            (string_iter.next(), string_iter.next())
                        {
                            if fn_name == "(" {
                                log!(
                                    LexerError,
                                    f("Expected function name at line {line_number}")
                                );
                                syntax_fn();
                            } else if fn_op_br != "(" {
                                log!(LexerError, f("Expected opening bracket after function name but got `{fn_op_br}` at line {line_number}"));
                                syntax_fn();
                            } else {
                                self.brackets.round += 1;
                                let mut arguments: Vec<String> = vec![];
                                while string_iter.peek().is_some() {
                                    if string_iter.peek().unwrap().as_str() == ")" {
                                        self.brackets.round -= 1;
                                        string_iter.next();
                                        break;
                                    } else if string_iter.peek().unwrap().as_str() == "(" {
                                        self.brackets.round += 1;
                                        log!(
                                            LexerError,
                                            f("Unexpected opening bracket at line {line_number}")
                                        );
                                    }
                                    arguments.push(string_iter.next().unwrap().to_string());
                                }
                                if let Some(op_braces) = string_iter.next() {
                                    if op_braces != "{" {
                                        log!(LexerError, f("Expected opening braces but found `{op_braces}` at line {line_number}"));
                                    } else {
                                        self.brackets.braces += 1;
                                        let mut fn_body: Vec<Vec<String>> = vec![];
                                        let mut function_parsed: bool = false;
                                        while line_iter.peek().is_some() {
                                            if function_parsed {
                                                break;
                                            }
                                            string_iter =
                                                line_iter.peek().unwrap().iter().peekable();
                                            while string_iter.peek().is_some() {
                                                let current_string = string_iter.next().unwrap();
                                                match current_string.as_str() {
                                                    "{" => {
                                                        line_iter.next();
                                                        self.brackets.braces += 1;
                                                    }
                                                    "}" => {
                                                        line_iter.next();
                                                        self.brackets.braces -= 1;
                                                        if self.brackets.braces == 0 {
                                                            if loop_ {
                                                                self.tmp_ast.push(
                                                                    Token::LoopFunction(Function {
                                                                        name: fn_name.to_owned(),
                                                                        arguments: arguments
                                                                            .clone(),
                                                                        lines: vec![],
                                                                        tmp_lines: fn_body.clone(),
                                                                        start_ln: line_number,
                                                                    }),
                                                                );
                                                                function_parsed = true;
                                                            } else {
                                                                self.tmp_ast.push(Token::Function(
                                                                    Function {
                                                                        name: fn_name.to_owned(),
                                                                        arguments: arguments
                                                                            .clone(),
                                                                        lines: vec![],
                                                                        tmp_lines: fn_body.clone(),
                                                                        start_ln: line_number,
                                                                    },
                                                                ));
                                                                function_parsed = true;
                                                            }
                                                            break;
                                                        } else {
                                                            fn_body.push(vec!["}".to_string()]);
                                                        }
                                                    }
                                                    _ => {
                                                        let temp =
                                                            line_iter.next().unwrap().to_owned();
                                                        if temp.contains(&"{".to_string()) {
                                                            self.brackets.braces += 1;
                                                        }
                                                        fn_body.push(temp);
                                                        string_iter = line_iter
                                                            .peek()
                                                            .unwrap()
                                                            .iter()
                                                            .peekable();
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    log!(Error, f("Expected opening braces at line {line_number}"));
                                }
                            }
                        } else {
                            log!(LexerError, f("Expected function name and opening bracket at line {line_number}"));
                            syntax_fn();
                        }
                    }
                    "#" => {
                        log!(Lexer, f("{:?}", next_line));
                        let comment = next_line.join(" ");
                        self.tmp_ast.push(Token::Comment(comment));
                    }
                    "const" | "global" => {
                        let syntax = || {
                            if str == "const" {
                                log!(Syntax, "const `name` = `value`");
                            } else {
                                log!(Syntax, "global `name` = `value`");
                            }
                        };
                        if let (Some(name), Some(equals), Some(value)) =
                            (string_iter.next(), string_iter.next(), string_iter.next())
                        {
                            if equals == "=" {
                                if str == "const" {
                                    self.tmp_ast.push(Token::Const(Let {
                                        name: name.to_owned(),
                                        value: value.to_owned(),
                                    }))
                                } else {
                                    self.tmp_ast.push(Token::Global(Let {
                                        name: name.to_owned(),
                                        value: value.to_owned(),
                                    }))
                                }
                            } else {
                                log!(LexerError, f("Expected `=` at line {line_number}"));
                                syntax();
                            }
                        } else {
                            log!(LexerError, f("Wrong Syntax at line {line_number}"));
                            syntax();
                        }
                    }
                    _ => {}
                }
            }
        }
        // ------------------------------
    }

    pub fn low_level(&mut self, code: Vec<Vec<String>>, start_ln: i32) -> Vec<Line> {
        let mut lines = vec![];
        let mut as_string: String;
        let mut line_iter = code.iter().peekable();
        let mut line_number = start_ln;
        while line_iter.peek().is_some() {
            let mut tokens = vec![];
            line_number += 1;
            let next_line = line_iter.next().unwrap();
            as_string = next_line.join(" ");
            let mut string_iter = next_line.iter().peekable();
            while string_iter.peek().is_some() {
                let string = string_iter.next().unwrap().as_str();
                match string {
                    "let" => {
                        let syntax = || {
                            log!(Syntax, "let `name` = `value`");
                        };
                        if let (Some(name), Some(equals), Some(value)) =
                            (string_iter.next(), string_iter.next(), string_iter.next())
                        {
                            if value.contains("[") && !value.contains("]") {
                                log!(LexerError, f("Creating a multiline Array with the let binding is not supported. Line: {line_number}"));
                            } else if equals == "=" {
                                tokens.push(Token::Var(Let {
                                    name: name.to_owned(),
                                    value: value.to_owned(),
                                }))
                            } else {
                                log!(LexerError, f("Expected `=` at line {line_number}"));
                                syntax();
                            }
                        } else {
                            log!(LexerError, f("Wrong Syntax at line {line_number}"));
                            syntax();
                        }
                    }
                    "if" => {
                        let syntax = || {
                            log!(Syntax, "\nif `condition` then\n   `code`\nend");
                        };
                        let mut condition_v: Vec<String> = vec![];
                        let mut then: bool = false;
                        while string_iter.peek().is_some() {
                            let nt = string_iter.next().unwrap();
                            if nt == "then" {
                                then = true;
                                self.brackets.braces += 1;
                                break;
                            }
                            condition_v.push(nt.to_owned());
                        }
                        if !then {
                            log!(LexerError, f("Expected then at line {line_number}"));
                            syntax();
                        } else {
                            if condition_v.is_empty() {
                                log!(LexerError, f("Expected consition at line {line_number}"));
                                syntax();
                            } else {
                                tokens.push(Token::If(If {
                                    condition: condition_v.join(" "),
                                    id: self.brackets.braces,
                                }));
                            }
                        }
                    }
                    "end" => {
                        tokens.push(Token::End(self.brackets.braces));
                        self.brackets.braces -= 1;
                    }
                    "[" => {
                        tokens.push(Token::OpenSqBr(self.brackets.square));
                        self.brackets.square += 1;
                    }
                    "]" => {
                        tokens.push(Token::OpenSqBr(self.brackets.square));
                        self.brackets.square -= 1;
                    }
                    "(" => {
                        tokens.push(Token::OpenRoBr(self.brackets.round));
                        self.brackets.round += 1;
                    }
                    ")" => {
                        tokens.push(Token::OpenRoBr(self.brackets.round));
                        self.brackets.round -= 1;
                    }
                    "," => {
                        tokens.push(Token::Comma);
                    }
                    _ => {
                        if let Some(nt) = string_iter.next() {
                            match nt.as_str() {
                                "=" => {
                                    let syntax = || {
                                        log!(Syntax, "`var` = `expr`");
                                    };
                                    //tokens.push(Token::Var(nt));
                                    if let Some(_) = string_iter.peek() {
                                    } else {
                                        log!(
                                            LexerError,
                                            f("Expected Expression at line {line_number}")
                                        );
                                        syntax();
                                    }
                                }
                                "(" => {}
                                _ => {}
                            }
                        } else {
                            log!(
                                LexerError,
                                f("Unexpected instruction at line {line_number}")
                            );
                        }
                    }
                }
            }
            lines.push(Line { tokens, as_string });
        }
        lines
    }

    pub fn parse_line() {}
}
