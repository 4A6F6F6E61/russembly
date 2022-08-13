use crate::{cpu::main::*, cpu::traits::*, cpu::CPUType};

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
