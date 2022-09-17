use colored::Colorize;

use crate::cpu::LEXER_ERROR_COUNT;
use {
    crate::{
        cpu::{jump_location::JumpLocation, CPUType},
        lexer::{Lexer, Line, Token},
        log,
    },
    conv::prelude::*,
    std::{
        fs::File,
        io::{self, BufRead},
        iter::Peekable,
        num::ParseIntError,
        path::Path,
        slice::Iter,
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
    pub name: String,
    pub value: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NumberVar {
    pub name: String,
    pub value: CPUType,
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
            stack: vec![],
            port: [0; 8],
            vars: vec![],
            accumulator: 0,
            jump_locations: vec![],
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

    #[allow(dead_code)]
    pub fn load_file(&mut self, path: &str) -> Option<Vec<Line>> {
        let mut lexer = Lexer::new();
        let mut line_count = 0;

        if let Ok(file) = self.read_lines(path) {
            line_count = file.count();
        }
        if let Ok(lines) = self.read_lines(path) {
            lines.for_each(|line| {
                let l = line.unwrap();
                lexer.run(l, line_count);
            });
            let mut lexer_error_c = 0usize;
            LEXER_ERROR_COUNT.with(|count| {
                lexer_error_c = *count.borrow();
            });
            log!(
                Lexer,
                f("Parsing the tokens returned {} errors", lexer_error_c)
            );
            if lexer_error_c != 0 {
                //exit(1)
            }
            log!(Info, "Finished parsing tokens");
        } else {
            log!(Error, "Unable to read lines");
        }
        //lexer.show_lines();
        lexer.get_lines()
    }

    #[allow(dead_code)]
    pub fn load_string(&mut self, string: &str) -> Option<Vec<Line>> {
        let mut lexer = Lexer::new();
        let code = &string.replace("~", "\n");
        let line_count = code.lines().count();

        if line_count != 0 {
            code.lines().for_each(|line| {
                let l = line.to_string();
                lexer.run(l, line_count);
            });
            let mut lexer_error_c = 0usize;
            LEXER_ERROR_COUNT.with(|count| {
                lexer_error_c = *count.borrow();
            });
            log!(
                Lexer,
                f("Parsing the tokens returned {} errors", lexer_error_c)
            );
            if lexer_error_c != 0 {
                //exit(1)
            }
            log!(Info, "Finished parsing tokens");
        } else {
            log!(Error, "Please provide some Code");
        }
        lexer.get_lines()
    }

    #[allow(dead_code)]
    fn read_lines<P>(&mut self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn get_port_from_str(&mut self, port_str: String) -> Result<usize, ParseIntError> {
        let mut chars = port_str.chars();
        chars.next();
        chars.as_str().parse::<usize>()
    }

    #[allow(dead_code)]
    pub fn get_json(&self) -> String {
        let mut output = String::new();
        output.push_str("{");
        //stack: vec![],
        output = format!("{}\"stack\":{:?},", output, self.stack);
        //port: [0; 8],
        output = format!("{}\"ports\":{{", output);
        self.port.iter().enumerate().for_each(|(i, port)| {
            if i == 0 {
                output = format!("{}\"{}\":\"{}\"", output, i, port);
            } else {
                output = format!("{},\"{}\":\"{}\"", output, i, port);
            }
        });
        output = format!("{}}},", output);
        //vars: vec![],
        output = format!("{}\"vars\":[", output,);
        self.vars.iter().enumerate().for_each(|(i, v)| {
            match v {
                Var::String(x) => {
                    if i == 0 {
                        output = format!(
                            "{}{{\"name\":{},\"value\":\"{}\"}}",
                            output, x.name, x.value
                        );
                    } else {
                        output = format!(
                            "{},{{\"name\":{},\"value\":\"{}\"}}",
                            output, x.name, x.value
                        );
                    }
                }
                Var::Number(x) => {
                    if i == 0 {
                        output = format!(
                            "{}{{\"name\":{},\"value\":\"{}\"}}",
                            output, x.name, x.value
                        );
                    } else {
                        output = format!(
                            ",{}{{\"name\":{},\"value\":\"{}\"}}",
                            output, x.name, x.value
                        );
                    }
                }
            }
            //output = format!("{}   {}: {},\n", output);
        });
        output.push_str("],");
        //accumulator: 0,
        output = format!("{}\"accumulator\":\"{}\"", output, self.accumulator);
        //jump_locations: vec![],
        output.push_str("}");
        return output;
    }

    pub fn try_get_var(&self, var: &str) -> Option<Var> {
        let mut out = Var::Number(NumberVar {
            name: "tmp".to_string(),
            value: 10,
        });
        let mut found: bool = false;
        self.vars.iter().for_each(|v| match v {
            Var::String(x) => {
                if x.name == var {
                    found = true;
                    out = Var::String(StringVar {
                        name: x.name.clone(),
                        value: x.value.clone(),
                    });
                }
            }
            Var::Number(x) => {
                if x.name == var {
                    found = true;
                    out = Var::Number(NumberVar {
                        name: x.name.clone(),
                        value: x.value.clone(),
                    });
                }
            }
        });
        if found {
            Some(out)
        } else {
            None
        }
    }

    pub fn cpu_line_error(
        &self,
        error: &str,
        line_string: String,
        line_number: usize,
        error_loc: usize,
        error_line: &str,
    ) {
        let line_len = format!("{}", line_number).len();
        let mut space = "".to_string();
        for _ in 0..=line_len {
            space.push(' ');
        }
        log!(Error, f("{}", error));
        let blue_line = "|".blue();
        let blue_line_number = format!("{}", line_number).blue();
        log!(Clear, f("{}{}", space, blue_line));
        log!(
            Clear,
            f("{} {} {}", blue_line_number, blue_line, line_string)
        );
        let mut temp = "".to_string();
        let mut arrows = "".to_string();
        temp.push_str(&format!("{}  ", blue_line));
        let strings: Vec<&str> = line_string.split(" ").collect();
        for i in 0..strings.len() {
            if i == error_loc {
                for _ in strings[i].chars() {
                    arrows.push('^');
                }
            } else {
                for _ in strings[i].chars() {
                    temp.push(' ');
                }
            }
        }
        let el_red = error_line.red();
        let arrows_red = arrows.red();
        log!(Clear, f("{}{}{} {}", space, temp, arrows_red, el_red));
        log!(Clear, f("{}{}\n", space, blue_line));
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

pub trait Run {
    fn run_lines(&mut self, lines: Vec<Line>);
    fn run_keywords(
        &mut self,
        token_iter: &mut Peekable<Iter<Token>>,
        token: &Token,
        line: String,
        line_number: usize,
    );
    fn run_opcodes(
        &mut self,
        token_iter: &mut Peekable<Iter<Token>>,
        token: &Token,
        line: String,
        line_number: usize,
    );
}
