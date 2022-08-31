use {
    crate::{
        cpu::{jump_location::JumpLocation, printx, CPUType, PrintT},
        lexer::{Lexer, Token, TokenType},
        log,
    },
    conv::prelude::*,
    std::{
        fs::File,
        io::{self, BufRead},
        num::ParseIntError,
        path::Path,
        process::exit,
    },
};

#[derive(Debug, Clone)]
pub enum Var {
    String(StringVar),
    Number(NumberVar),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StringVar {
    name: String,
    value: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NumberVar {
    name: String,
    value: CPUType,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CPU<CPUType> {
    pub stack: Vec<CPUType>,
    pub port: [CPUType; 8],
    pub vars: Vec<Var>,
    pub accumulator: CPUType,
    pub jump_locations: Vec<JumpLocation>,
    pub error_count: usize,
}

impl CPU<CPUType> {
    pub fn new<'t>() -> Result<Self, &'t str> {
        Ok(CPU {
            stack: Vec::new(),
            port: [0; 8],
            vars: Vec::new(),
            accumulator: 0,
            jump_locations: Vec::new(),
            error_count: 0,
        })
    }
    pub fn push_to_stack(&mut self, value: CPUType) {
        self.stack.push(value);
    }
    pub fn pop_from_stack(&mut self) -> Option<usize> {
        self.stack.pop()
    }
    #[allow(dead_code)]
    pub fn add_jump_location(&mut self, name: String, line: usize) {
        self.jump_locations.push(JumpLocation { name, line })
    }

    pub fn load_file(&mut self, path: &str) -> Result<Vec<Token>, ()> {
        let mut lexer = Lexer::new();
        let mut lexer_error_c = 0usize;
        let mut line_count = 0;

        if let Ok(file) = self.read_lines(path) {
            line_count = file.count();
        }
        if let Ok(lines) = self.read_lines(path) {
            lines.for_each(|line| {
                let l = line.unwrap();
                lexer_error_c += lexer.run(l, line_count);
            });
            log!(
                Lexer,
                f("Parsing the tokens returned {} errors", lexer_error_c)
            );
            if lexer_error_c != 0 {
                exit(1)
            }
            log!(Info, "Finished parsing tokens");
        } else {
            log!(Error, "Unable to read lines");
        }
        lexer.get_tokens()
    }

    fn read_lines<P>(&mut self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn run_tokens(&mut self, tokens: Vec<Token>) {
        let mut error_count = 0usize;
        println!("\nOutput:");
        println!("-----------------------------");
        let mut token_iter = tokens.iter().peekable();

        while token_iter.peek().is_some() {
            let token = token_iter.next().unwrap();
            match &token.token_type {
                TokenType::OpCode => match token.value.as_str() {
                    "push" => match token_iter.next().unwrap().token_type {
                        TokenType::Number(x) => self.stack.push(x),
                        _ => {
                            error_count += 1;
                            log!(Error, "You can only push Numbers to the Stack!");
                        }
                    },
                    "pop" => {
                        self.stack.pop();
                    }
                    // move value
                    "mov" => {
                        let port_or_accu = token_iter.next().unwrap();
                        match token_iter.next().unwrap().token_type {
                            // check for comma
                            TokenType::Comma => {}
                            _ => {
                                error_count += 1;
                                log!(Error, "Expected Comma");
                            }
                        }
                        let value = token_iter.next().unwrap();
                        match port_or_accu.token_type {
                            // push to port
                            TokenType::Port => {}
                            // push to accumulator
                            TokenType::Accumulator => {
                                self.accumulator = match value.token_type {
                                    TokenType::Number(x) => x,
                                    TokenType::Port => {
                                        let port =
                                            self.get_port_from_str(value.value.clone()).unwrap(); // get the Port number
                                        self.port[port]
                                    }
                                    _ => {
                                        error_count += 1;
                                        log!(Error,"You can only move a number or the value of a Port to the Accumulator");
                                        continue;
                                    }
                                }
                            }
                            _ => {
                                error_count += 1;
                                log!(Error, "Expected Port or Accu!");
                            }
                        }
                    }
                    // add top 2 number from stack together and push them on the stack
                    "adds" => {
                        let a = self.stack.pop().unwrap();
                        let b = self.stack.pop().unwrap();
                        self.stack.push(a + b);
                    }
                    // sub top 2 number from stack together and push them on the stack
                    "subs" => {
                        let a = self.stack.pop().unwrap();
                        let b = self.stack.pop().unwrap();
                        self.stack.push(a - b);
                    }
                    // mul top 2 number from stack together and push them on the stack
                    "muls" => {
                        let a = self.stack.pop().unwrap();
                        let b = self.stack.pop().unwrap();
                        self.stack.push(a * b);
                    }
                    // div top 2 number from stack together and push them on the stack
                    "divs" => {
                        let a = self.stack.pop().unwrap();
                        let b = self.stack.pop().unwrap();
                        self.stack.push(a / b);
                    }
                    "djnz" => {}
                    "jmp" => {}
                    "setb" => {}
                    "end" => {}
                    // print given string or number
                    "prnt" => match token_iter.peek().unwrap().token_type {
                        TokenType::String => {
                            print!("{}", token_iter.next().unwrap().value)
                        }
                        TokenType::Number(x) => {
                            token_iter.next();
                            print!("{}", x)
                        }
                        _ => {
                            error_count += 1;
                            log!(Error, "Print only accepts Strings and Numbers");
                        }
                    },
                    "call" => {}
                    &_ => {}
                },
                TokenType::JumpLocation(_jump_location) => {}
                TokenType::Bracket => {}
                TokenType::Keyword => {
                    match token.value.as_str() {
                        "fn" => {
                            token_iter.next();
                            token_iter.next();
                        }
                        "let" => {
                            let mut nt = match token_iter.next() {
                                Some(x) => x,
                                None => {
                                    error_count += 1;
                                    log!(Error, "Expected Arguments after let");
                                    continue;
                                }
                            };
                            let var_name = match nt.token_type {
                                TokenType::VarName => nt.value.as_str(),
                                _ => {
                                    error_count += 1;
                                    log!(Error, "Expected variable name");
                                    continue;
                                }
                            };
                            match token_iter.next().expect("Error: Expected Comma").token_type {
                                TokenType::Comma => { /* Do nothing, just for checking*/ }
                                _ => {
                                    error_count += 1;
                                    log!(Error, "Expected Comma");
                                }
                            }
                            nt = token_iter.next().expect("Error: Expected value for let");
                            match nt.token_type {
                                TokenType::String => {
                                    self.vars.push(Var::String(StringVar {
                                        name: var_name.to_string(),
                                        value: nt.clone().value,
                                    }));
                                }
                                TokenType::Number(x) => {
                                    self.vars.push(Var::Number(NumberVar {
                                        name: var_name.to_string(),
                                        value: x,
                                    }));
                                }
                                _ => {
                                    printx(
                                        PrintT::Error,
                                        "You can only store Strings and Numbers inside a Variable",
                                    );
                                }
                            }
                        }
                        _ => {}
                    }
                }
                TokenType::String => {}
                TokenType::Comment => {}
                // Prints a new Line
                TokenType::NewLine => {
                    println!("")
                }
                _ => {
                    printx(
                        PrintT::Error,
                        format!("unexpected token '{}' at line {}", token.value, token.line)
                            .as_str(),
                    );
                }
            }
        }
        log!(
            Cpu,
            f("Interpreting the tokens returned {} errors", error_count)
        );
        println!("-----------------------------\n");
    }
    fn get_port_from_str(&mut self, port_str: String) -> Result<usize, ParseIntError> {
        let mut chars = port_str.chars();
        chars.next();
        chars.as_str().parse::<usize>()
    }
}
/* Traits
   - OpCodes
   - ShowCPU
   - CpuGetter
*/

pub trait OpCodes<CPUType> {
    // Move value into Port
    fn mov<T>(&mut self, port: usize, value: T)
    where
        CPUType: ValueFrom<T>;
    // move value on the Accumulator
    fn mova<T>(&mut self, value: T)
    where
        CPUType: ValueFrom<T>;
    // move value from Port to Accumulator
    fn mova_p(&mut self, port: usize);
    // add top 2 number from stack together and push them on the stack
    fn add(&mut self);
    // sub top 2 number from stack together and push them on the stack
    fn sub(&mut self);
    // mul top 2 number from stack together and push them on the stack
    fn mul(&mut self);
    // div top 2 number from stack together and push them on the stack
    fn div(&mut self);
    // add value from Port to Accumulator
    fn addp(&mut self, port: usize);
    // subtract value from Port to Accumulator
    fn subp(&mut self, port: usize);
    // decrement and jump if not zero
    fn djnz(&mut self, port: usize, jmp_loc_name: String);
    // jump to jmp_location
    fn jmp(&mut self, jmp_loc_name: String);
    // set Bit to 1
    fn setb(&mut self, port_bit: String);
}

pub trait ShowCPU {
    fn show_cpu(&self);
    fn show_stack(&self);
    fn show_port(&self);
    fn show_jump_locations(&self);
    fn show_vars(&self);
}

pub trait CpuGetter<CPUType> {
    fn get_stack(&self) -> &Vec<CPUType>;
    fn get_port(&self, port: usize) -> CPUType;
    fn get_accumulator(&self) -> &CPUType;
    fn get_jump_locations(&self) -> &Vec<JumpLocation>;
}
