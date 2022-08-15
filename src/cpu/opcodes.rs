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
    fn setb(&mut self, _port: usize, _bit: usize) {
        //self.port[port] |= 1 << bit;
    }
}
