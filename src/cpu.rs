use conv::prelude::*;

type CPUType = f64;

#[derive(Debug, Clone)]
pub struct JumpLocation {
    pub name: String,
    pub line: usize,
}

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
    fn setb(&mut self, port: usize, bit: usize);
}

pub trait ShowCPU {
    fn show_cpu(&self);
    fn show_stack(&self);
    fn show_port(&self);
    fn show_jump_locations(&self);
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CPU<CPUType> {
    pub stack: Vec<CPUType>,
    pub port: [CPUType; 8],
    pub accumulator: CPUType,
    pub jump_locations: Vec<JumpLocation>,
}

impl CPU<CPUType> {
    pub fn new() -> Self {
        CPU {
            stack: Vec::new(),
            port: [0.0; 8],
            accumulator: 0.0,
            jump_locations: Vec::new(),
        }
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

impl OpCodes<CPUType> for CPU<CPUType> {
    fn mov<T>(&mut self, port: usize, value: T)
    where
        CPUType: ValueFrom<T>,
    {
        self.port[port] = value.value_as::<CPUType>().unwrap();
    }
    fn mova<T>(&mut self, value: T)
    where
        CPUType: ValueFrom<T>,
    {
        self.accumulator = value.value_as::<CPUType>().unwrap();
    }
    fn mova_p(&mut self, port: usize) {
        self.accumulator = self.port[port];
    }
    fn add(&mut self) -> () {
        let a = self.pop_from_stack();
        let b = self.pop_from_stack();
        self.push_to_stack(a + b);
    }
    fn sub(&mut self) -> () {
        let a = self.pop_from_stack();
        let b = self.pop_from_stack();
        self.push_to_stack(a - b);
    }
    fn mul(&mut self) -> () {
        let a = self.pop_from_stack();
        let b = self.pop_from_stack();
        self.push_to_stack(a * b);
    }
    fn div(&mut self) -> () {
        let a = self.pop_from_stack();
        let b = self.pop_from_stack();
        self.push_to_stack(a / b);
    }
    fn addp(&mut self, port: usize) {
        self.accumulator += self.port[port];
    }
    fn subp(&mut self, port: usize) {
        self.accumulator -= self.port[port];
    }
    fn djnz(&mut self, port: usize, jmp_loc_name: String) {
        if self.port[port] != 0.0 {
            self.port[port] -= 1.0;
            self.jmp(jmp_loc_name);
        }
    }
    fn jmp(&mut self, _jmp_loc_name: String) {
        /* TODO */
    }
    fn setb(&mut self, port: usize, bit: usize) {
        //self.port[port] |= 1 << bit;
    }
}

impl ShowCPU for CPU<CPUType> {
    fn show_cpu(&self) {
        self.show_stack();
        self.show_port();
        println!("Accumulator: {}", self.get_accumulator());
        self.show_jump_locations();
    }
    fn show_stack(&self) {
        println!("Stack: {:?}", self.stack);
    }
    fn show_port(&self) {
        print!("Port:                {{ ");
        self.port.iter().enumerate().for_each(|(i, x)| {
            if i == 0 {
                print!("P{}: {}", i, x);
            } else {
                print!(", P{}: {}", i, x);
            }
        });
        println!(" }}");
    }
    fn show_jump_locations(&self) {
        println!("Jump Locations: {{");
        self.jump_locations.iter().for_each(|x| {
            println!("    {:?}", x);
        });
        println!("}}");
    }
}

pub trait CpuGetter<CPUType> {
    fn get_stack(&self) -> &Vec<CPUType>;
    fn get_port(&self) -> &[CPUType; 8];
    fn get_accumulator(&self) -> &CPUType;
    fn get_jump_locations(&self) -> &Vec<JumpLocation>;
}

impl CpuGetter<CPUType> for CPU<CPUType> {
    fn get_stack(&self) -> &Vec<CPUType> {
        &self.stack
    }
    fn get_port(&self) -> &[CPUType; 8] {
        &self.port
    }
    fn get_accumulator(&self) -> &CPUType {
        &self.accumulator
    }
    fn get_jump_locations(&self) -> &Vec<JumpLocation> {
        &self.jump_locations
    }
}
