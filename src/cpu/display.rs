use {
    crate::cpu::{main::CPU, CPUType},
    std::fmt::{Display, Formatter, Result},
};

impl Display for CPU<CPUType> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut output: String = String::new();
        output.push_str(&format!("Stack:          {:?}\n", self.stack));
        // Port
        output.push_str(&format!("Port:           {{\n"));
        self.port.iter().enumerate().for_each(|(i, x)| {
            if i == 0 {
                output.push_str(&format!("      P{}: 0x{:x}", i, x));
            } else {
                output.push_str(&format!(", P{}: 0x{:x}", i, x));
            }
        });
        // ln
        output.push_str(&format!("\n}}\n"));
        // JumpLocations
        output.push_str(&format!("Jump Locations: {{\n"));
        self.jump_locations.iter().for_each(|x| {
            output.push_str(&format!("    {:?}\n", x));
        });
        output.push_str(&format!("}}\n"));
        // Accu
        output.push_str(&format!("Accumulator:    {:?}\n", self.get_accumulator()));
        // Vars
        output.push_str(&format!("Vars: {{\n"));
        self.vars.iter().for_each(|x| {
            output.push_str(&format!("    {:?}\n", x));
        });
        output.push_str(&format!("}}\n"));
        write!(f, "{}", output)
    }
}
