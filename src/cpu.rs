#[derive(Debug, Clone)]
pub struct JumpLocation {
    pub name: String,
    pub line:  usize,
}

pub trait OpCodes {
    // move value on the given register
    fn mov(&mut self, register: usize, value: i64);
    // move value to the given floating register
    fn mov_f(&mut self, f_register: usize, value: f64);
    // add top 2 number from stack together and push them on the stack
    fn add(&mut self);
    // sub top 2 number from stack together and push them on the stack
    fn sub(&mut self);
    // mul top 2 number from stack together and push them on the stack
    fn mul(&mut self);
    // div top 2 number from stack together and push them on the stack
    fn div(&mut self);
    // add value to register
    fn addi(&mut self, register: usize, value: i64);
    // add value to Float register
    fn addf(&mut self, f_register: usize, value: f64);
    // add two registers and put the result on the first register given
    fn addi_r(&mut self, register_1: usize, register_2: usize);
    // add two floating registers and put the result on the first floating register given
    fn addf_r(&mut self, f_register_1: usize, f_register_2: usize);
    // decrement and jump if not zero
    fn djnz(&mut self, register: usize, jmp_loc_name: String);
    // jump to jmp_location
    fn jmp(&mut self, jmp_loc_name: String);
}

pub trait ShowCPU {
    fn show_cpu(&self);
    fn show_stack(&self);
    fn show_register(&self);
    fn show_floating_point_register(&self);
    fn show_jump_locations(&self);
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CPU {
    pub stack: Vec<i64>,
    pub register: [i64; 8],
    pub floating_point_register: [f64; 8],
    pub jump_locations: Vec<JumpLocation>,
}
impl CPU {
    pub fn new() -> Self {
        CPU {
            stack:                   Vec::new(),
            register:                [0; 8],
            floating_point_register: [0.0; 8],
            jump_locations:           Vec::new(),
        }
    }
    pub fn push_to_stack(&mut self, value: i64) {
        self.stack.push(value);
    }
    pub fn pop_from_stack(&mut self) -> i64 {
        self.stack.pop().unwrap()
    }
    pub fn add_jump_location(&mut self, jump_loc_name: String, line: usize) {
        self.jump_locations.push(JumpLocation { name: jump_loc_name, line })
    }
}

impl OpCodes for CPU {
    fn mov(&mut self, register: usize, value: i64) {
        self.register[register] = value;
    }
    fn mov_f(&mut self, f_register: usize, value: f64) {
        self.floating_point_register[f_register] = value;
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
    fn addi(&mut self, register: usize, value: i64) {
        self.register[register] += value;
    }
    fn addf(&mut self, f_register: usize, value: f64) {
        self.floating_point_register[f_register] += value;
    }
    fn addi_r(&mut self, register_1: usize, register_2: usize) {
        self.register[register_1] += self.register[register_2];
    }
    fn addf_r(&mut self, f_register_1: usize, f_register_2: usize) {
        self.floating_point_register[f_register_1] += self.floating_point_register[f_register_2];
    }
    fn djnz(&mut self, register: usize, jmp_loc_name: String) {
        if self.register[register] != 0 {
            self.register[register] -= 1;
            self.jmp(jmp_loc_name);
        }
    }
    fn jmp(&mut self, _jmp_loc_name: String) {
        // todo!()
    }
}

impl ShowCPU for CPU {
    fn show_cpu(&self) {
        self.show_stack();
        self.show_register();
        self.show_floating_point_register();
        self.show_jump_locations();
    }
    fn show_stack(&self) {
        println!("Stack: {:?}", self.stack);
    }
    fn show_register(&self) {
        print!("Register:                {{ ");
        self.register.iter().enumerate().for_each(|(i, x)| {
            if i == 0 {
                print!("R{}: {}", i, x);
            } else {
                print!(", R{}: {}", i, x);
            }
        });
        println!(" }}");
    }
    fn show_floating_point_register(&self) {
        print!("Floating Point Register: {{ ");
        self.floating_point_register
            .iter()
            .enumerate()
            .for_each(|(i, x)| {
                if i == 0 {
                    print!("R{}: {}", i, x);
                } else {
                    print!(", R{}: {}", i, x);
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
