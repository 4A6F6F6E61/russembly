use crate::{
    cpu::{main::*, printx, CPUType, PrintT},
    lexer::{Line, Token, TokenType},
    log,
};
use std::iter::Peekable;
use std::slice::Iter;

impl Run for CPU<CPUType> {
    fn run_lines(&mut self, lines: Vec<Line>) {
        let mut error_count = 0usize;
        self.output = vec![];
        printx(PrintT::Clear, "\nOutput:");
        printx(PrintT::Clear, "-----------------------------");
        for i in 0..(lines.len()) {
            let mut token_iter = lines[i].tokens.iter().peekable();

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
                            format!("unexpected token '{}' at line {}", token.value, i).as_str(),
                        );
                    }
                }
            }
        }
        printx(PrintT::Clear, "-----------------------------\n");
        log!(
            Cpu,
            f("Interpreting the tokens returned {} errors", error_count)
        );
    }

    fn run_keywords(
        &mut self,
        error_count: &mut usize,
        token_iter: &mut Peekable<Iter<Token>>,
        token: &Token,
    ) {
        match token.value.as_str() {
            "fn" => {
                token_iter.next();
                token_iter.next();
            }
            "let" => {
                let nt = match token_iter.next() {
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
                if let Some(nt) = token_iter.next() {
                    match nt.token_type {
                        TokenType::Comma => { /* Do nothing, just for checking*/ }
                        _ => {
                            *error_count += 1;
                            log!(Error, "Expected Comma");
                        }
                    }
                } else {
                    *error_count += 1;
                    log!(Error, "Expected Comma");
                }
                if let Some(nt) = token_iter.next() {
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
                } else {
                    *error_count += 1;
                    log!(Error, "Expected value for let");
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
            "push" => {
                if let Some(nt) = token_iter.next() {
                    match nt.token_type {
                        TokenType::Number(x) => self.stack.push(x),
                        _ => {
                            *error_count += 1;
                            log!(Error, "You can only push Numbers to the Stack!");
                        }
                    }
                } else {
                    *error_count += 1;
                    log!(Error, "Expected Number after push");
                }
            }
            "pop" => {
                self.stack.pop();
            }
            // move value
            "mov" => {
                if let (Some(port_or_accu), Some(comma), Some(value)) =
                    (token_iter.next(), token_iter.next(), token_iter.next())
                {
                    match comma.token_type {
                        // check for comma
                        TokenType::Comma => {}
                        _ => {
                            *error_count += 1;
                            log!(Error, "Expected Comma");
                        }
                    }
                    match port_or_accu.token_type {
                        // push to port
                        TokenType::Port => {
                            if let Ok(number) = self.get_port_from_str(port_or_accu.value.clone()) {
                                self.port[number] = match value.token_type {
                                    TokenType::Number(x) => x,
                                    TokenType::Port => {
                                        let port =
                                            self.get_port_from_str(value.value.clone()).unwrap(); // get the Port number
                                        self.port[port]
                                    }
                                    _ => {
                                        *error_count += 1;
                                        log!(Error,"You can only move a number or the value of a Port to this Port");
                                        return;
                                    }
                                }
                            }
                        }
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
                } else {
                    *error_count += 1;
                    log!(Error, "Expected more Tokens after mov");
                    log!(Syntax, "mov <Port or Accu> <,> <value>");
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
            "prnt" => {
                if let Some(nt) = token_iter.next() {
                    match nt.token_type {
                        TokenType::String => {
                            let v = &nt.value;
                            printx(PrintT::Clear, &format!("{}", v));
                        }
                        TokenType::Number(x) => {
                            printx(PrintT::Clear, &format!("{}", x));
                        }
                        TokenType::Accumulator => {
                            printx(PrintT::Clear, &format!("{}", self.accumulator));
                        }
                        TokenType::Stack => {
                            printx(PrintT::Clear, &format!("{:?}", self.stack));
                        }
                        TokenType::Port => {
                            if let Ok(port) = self.get_port_from_str(nt.value.clone()) {
                                printx(PrintT::Clear, &format!("{}", self.port[port]));
                            } else {
                                *error_count += 1;
                                log!(Error, "Invalid Port");
                            }
                        }
                        _ => {
                            *error_count += 1;
                            log!(Error, "Print only accepts Strings and Numbers");
                        }
                    }
                } else {
                    *error_count += 1;
                    log!(Error, "Expected Token after print Statement");
                }
            }
            "call" => {}
            &_ => {}
        }
    }
}
