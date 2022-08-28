use crate::{cpu::main::*, cpu::CPUType};
use conv::prelude::*;

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
        let a = self.pop_from_stack().unwrap();
        let b = self.pop_from_stack().unwrap();
        self.push_to_stack(a + b);
    }
    fn sub(&mut self) -> () {
        let a = self.pop_from_stack().unwrap();
        let b = self.pop_from_stack().unwrap();
        self.push_to_stack(a - b);
    }
    fn mul(&mut self) -> () {
        let a = self.pop_from_stack().unwrap();
        let b = self.pop_from_stack().unwrap();
        self.push_to_stack(a * b);
    }
    fn div(&mut self) -> () {
        let a = self.pop_from_stack().unwrap();
        let b = self.pop_from_stack().unwrap();
        self.push_to_stack(a / b);
    }
    fn addp(&mut self, port: usize) {
        self.accumulator += self.port[port];
    }
    fn subp(&mut self, port: usize) {
        self.accumulator -= self.port[port];
    }
    fn djnz(&mut self, port: usize, jmp_loc_name: String) {
        if self.port[port] != 0 {
            self.port[port] -= 1;
            self.jmp(jmp_loc_name);
        }
    }
    fn jmp(&mut self, _jmp_loc_name: String) {
        /* TODO */
    }
    fn setb(&mut self, port_bit: String) {
        let s = port_bit.split("^");
        let vec = s.collect::<Vec<&str>>();
        let mut chars = vec[0].chars();
        chars.next();
        match (chars.as_str().parse::<usize>(), vec[1].parse::<usize>()) {
            (Ok(port), Ok(bit)) => {
                if port >= 7 {
                    println!("Port: {} out of bounds (0 - 7)", port);
                    return;
                }
                if bit > 63 {
                    println!(
                        "Setting the {}th bit will lead to a stack overflow (max is 63)",
                        bit
                    );
                    return;
                }
                self.port[port] |= 1 << bit;
            }
            _ => println!("Error parsing: {}", port_bit),
        }
    }
}
