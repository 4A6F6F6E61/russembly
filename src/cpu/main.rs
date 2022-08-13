use crate::{cpu::jump_location::JumpLocation, cpu::CPUType};

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
            port: [0.0; 8],
            accumulator: 0.0,
            jump_locations: Vec::new(),
        })
    }
    pub fn push_to_stack(&mut self, value: CPUType) {
        self.stack.push(value);
    }
    pub fn pop_from_stack(&mut self) -> CPUType {
        self.stack.pop().unwrap()
    }
    pub fn add_jump_location(&mut self, jump_loc_name: String, line: usize) {
        self.jump_locations.push(JumpLocation {
            name: jump_loc_name,
            line,
        })
    }
}
