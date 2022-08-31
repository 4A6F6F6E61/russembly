use crate::{cpu::main::*, cpu::CPUType};

impl ShowCPU for CPU<CPUType> {
    fn show_cpu(&self) {
        self.show_stack();
        self.show_port();
        println!("Accumulator:    {}", self.get_accumulator());
        self.show_jump_locations();
        self.show_vars();
    }
    fn show_stack(&self) {
        println!("Stack:          {:?}", self.stack);
    }
    fn show_port(&self) {
        print!("Port:           {{ ");
        self.port.iter().enumerate().for_each(|(i, x)| {
            if i == 0 {
                print!("P{}: 0x{:x}", i, x);
            } else {
                print!(", P{}: 0x{:x}", i, x);
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
    fn show_vars(&self) {
        println!("Vars: {{");
        self.vars.iter().for_each(|x| {
            println!("    {:?}", x);
        });
        println!("}}");
    }
}
