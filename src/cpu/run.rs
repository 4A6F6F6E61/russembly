use std::iter::Peekable;
use std::slice::Iter;
use {
    crate::{
        cpu::{printx, CPUType, PrintT, main::*},
        lexer::{Token, TokenType},
        log,
    },
};

impl Run for CPU<CPUType> {
    fn run_tokens(&mut self, tokens: Vec<Token>) {
        let mut error_count = 0usize;
        println!("\nOutput:");
        println!("-----------------------------");
        let mut token_iter = tokens.iter().peekable();

        while token_iter.peek().is_some() {
            let token = token_iter.next().unwrap();
            match &token.token_type {
                TokenType::OpCode => self.run_opcodes(&mut error_count, &mut token_iter, token),
                TokenType::JumpLocation(_jump_location) => {}
                TokenType::Bracket => {}
                TokenType::Keyword => {
                    self.run_keywords(&mut error_count, &mut token_iter, token)
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

    fn run_keywords(&mut self, error_count: &mut usize, token_iter: &mut Peekable<Iter<Token>>, token: &Token) {
        match token.value.as_str() {
            "fn" => {
                token_iter.next();
                token_iter.next();
            }
            "let" => {
                let mut nt = match token_iter.next() {
                    Some(x) => x,
                    None => {
                        *error_count += 1;
                        log!(Error, "Expected Arguments after let");
                        return;
                    }
                };
                let var_name = match nt.token_type {
                    TokenType::VarName => nt.value.as_str(),
                    _ => {
                        *error_count += 1;
                        log!(Error, "Expected variable name");
                        return;
                    }
                };
                match token_iter.next().expect("Error: Expected Comma").token_type {
                    TokenType::Comma => { /* Do nothing, just for checking*/ }
                    _ => {
                        *error_count += 1;
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

    fn run_opcodes(
        &mut self,
        error_count: &mut usize,
        token_iter: &mut Peekable<Iter<Token>>,
        token: &Token,
    ) {
        match token.value.as_str() {
            "push" => match token_iter.next().unwrap().token_type {
                TokenType::Number(x) => self.stack.push(x),
                _ => {
                    *error_count += 1;
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
                        *error_count += 1;
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
                                let port = self.get_port_from_str(value.value.clone()).unwrap(); // get the Port number
                                self.port[port]
                            }
                            _ => {
                                *error_count += 1;
                                log!(Error,"You can only move a number or the value of a Port to the Accumulator");
                                return;
                            }
                        }
                    }
                    _ => {
                        *error_count += 1;
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
                    *error_count += 1;
                    log!(Error, "Print only accepts Strings and Numbers");
                }
            },
            "call" => {}
            &_ => {}
        }
    }
}