use std::iter::Peekable;
use std::slice::Iter;

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
        num::ParseIntError,
        path::Path,
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
        output.push_str("{\n");
        //stack: vec![],
        output = format!("{} stack: {:?},\n", output, self.stack);
        //port: [0; 8],
        output = format!("{} ports: {{\n", output);
        self.port.iter().enumerate().for_each(|(i, port)| {
            output = format!("{}   {}: {},\n", output, i, port);
        });
        output = format!("{} }},\n", output);
        //vars: vec![],
        output = format!("{} vars: [ \n", output,);
        self.vars.iter().for_each(|v| {
            match v {
                Var::String(x) => {
                    output = format!(
                        "{}    {{name: {}, value: \"{}\"}},\n",
                        output, x.name, x.value
                    );
                }
                Var::Number(x) => {
                    output = format!("{}    {{name: {}, value: {}}},\n", output, x.name, x.value);
                }
            }
            //output = format!("{}   {}: {},\n", output);
        });
        output.push_str(" ],\n");
        //accumulator: 0,
        output = format!("{} accumulator: {},\n", output, self.accumulator);
        //jump_locations: vec![],
        //error_count: 0,
        output = format!("{} error_count: {},\n", output, self.accumulator);
        //ex: String::new(),
        output.push_str("}");
        return output;
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
    fn run_keywords(&mut self, token_iter: &mut Peekable<Iter<Token>>, token: &Token);
    fn run_opcodes(&mut self, token_iter: &mut Peekable<Iter<Token>>, token: &Token);
}
