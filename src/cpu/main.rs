use crate::lexer::{Lexer, LexerError, Token, TokenType};
use crate::{cpu::jump_location::JumpLocation, cpu::CPUType};
use conv::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::Path;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CPU<CPUType> {
    pub stack: Vec<CPUType>,
    pub port: [CPUType; 8],
    pub accumulator: CPUType,
    pub jump_locations: Vec<JumpLocation>,
}

impl CPU<CPUType> {
    pub fn new<'t>() -> Result<Self, &'t str> {
        Ok(CPU {
            stack: Vec::new(),
            port: [0; 8],
            accumulator: 0,
            jump_locations: Vec::new(),
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

    pub fn load_file(&mut self, path: &str) -> Result<Vec<Token>, LexerError> {
        let mut lexer = Lexer::new();
        if let Ok(lines) = self.read_lines(path) {
            lines.for_each(|line| {
                let l = line.unwrap();
                lexer.run(l).expect("Unable to parse line");
            });
        }
        lexer.clone().show_tokens();
        lexer.get_tokens()
    }

    fn read_lines<P>(&mut self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn run_tokens(&mut self, tokens: Vec<Token>) -> Result<(), &str> {
        let mut token_iter = tokens.iter().peekable();

        while token_iter.peek().is_some() {
            let token = token_iter.next().unwrap();
            match &token.token_type {
                TokenType::OpCode => match token.value.as_str() {
                    "push" => match token_iter.next().unwrap().token_type {
                        TokenType::Number(x) => self.stack.push(x),
                        _ => {
                            return Err("Error: You can only push Numbers to the Stack!");
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
                                return Err("Error: Expected Comma");
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
                                        let port = self.get_port_from_str(value.value.clone()).unwrap(); // get the Port number
                                        self.port[port]
                                    }
                                    _ => return Err("Error: You can only move a number or the value of a Port to the Accumulator")
                                }
                            }
                            _ => {
                                return Err("Error: Expected Port or Accu!");
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
                            print!("{}", x)
                        }
                        _ => {
                            return Err("Error: Print only accepts Strings and Numbers");
                        }
                    },
                    "call" => {}
                    &_ => {}
                },
                TokenType::Accumulator => {}
                TokenType::Port => {}
                TokenType::JumpLocation(_jump_location) => {}
                TokenType::FunctionName => {}
                TokenType::Bracket => {}
                TokenType::Keyword => {}
                TokenType::String => {}
                TokenType::Number(_) => {}
                TokenType::Comment => {}
                TokenType::Comma => {}
                // Prints a new Line
                TokenType::NewLine => {
                    println!("")
                }
            }
        }
        Ok(())
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
}

pub trait CpuGetter<CPUType> {
    fn get_stack(&self) -> &Vec<CPUType>;
    fn get_port(&self, port: usize) -> CPUType;
    fn get_accumulator(&self) -> &CPUType;
    fn get_jump_locations(&self) -> &Vec<JumpLocation>;
}
