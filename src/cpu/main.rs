use crate::lexer::Lexer;
use crate::{cpu::jump_location::JumpLocation, cpu::CPUType};
use conv::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
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
    pub fn pop_from_stack(&mut self) -> Result<CPUType, String> {
        Ok(self
            .stack
            .pop()
            .expect("Unable to pop from stack. \nTip: Check if it's empty"))
        /*match self.stack.pop() {
            Ok(x) => Ok(x),
            Err => Result::Err("Unable to pop from stack. \nTip: Check if it's empty".to_string())
        }*/
    }
    pub fn add_jump_location(&mut self, name: String, line: usize) {
        self.jump_locations.push(JumpLocation { name, line })
    }

    pub fn load_file(&mut self, path: &str) -> Result<(), String> {
        let mut lexer = Lexer::new();
        if let Ok(lines) = self.read_lines(path) {
            for line in lines {
                let l = line.unwrap();
                print!("{}", l);
                lexer.run(l);
            }
        }
        println!("{:?}", lexer);
        Ok(())
    }

    fn read_lines<P>(&mut self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
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
