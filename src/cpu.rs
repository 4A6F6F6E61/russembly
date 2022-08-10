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
}

pub trait ShowCPU {
    fn show_cpu(&self);
    fn show_stack(&self);
    fn show_register(&self);
    fn show_floating_point_register(&self);
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CPU {
    pub stack: Vec<i64>,
    pub register: [i64; 8],
    pub floating_point_register: [f64; 8],
}
impl CPU {
    pub fn new() -> Self {
        CPU {
            stack: Vec::new(),
            register: [0; 8],
            floating_point_register: [0.0; 8],
        }
    }
    pub fn push_to_stack(&mut self, value: i64) {
        self.stack.push(value);
    }
    pub fn pop_from_stack(&mut self) -> i64 {
        self.stack.pop().unwrap()
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
}

impl ShowCPU for CPU {
    fn show_cpu(&self) {
        self.show_stack();
        self.show_register();
        self.show_floating_point_register()
    }
    fn show_stack(&self) {
        println!("Stack: {:?}", self.stack);
    }
    fn show_register(&self) {
        print!("Register: ");
        print!("{{ ");
        self.register.iter().enumerate().for_each(|(i, x)| {
            if i == 0 {
                print!("{}", x);
            } else {
                print!(", {}", x);
            }
        });
        println!(" }}");
    }
    fn show_floating_point_register(&self) {
        print!("Floating Point Register: ");
        print!("{{ ");
        self.floating_point_register
            .iter()
            .enumerate()
            .for_each(|(i, x)| {
                if i == 0 {
                    print!("{}", x);
                } else {
                    print!(", {}", x);
                }
            });
        println!(" }}");
    }
}
