use crate::{cpu::jump_location::JumpLocation, cpu::main::*, cpu::CPUType};

impl CpuGetter<CPUType> for CPU<CPUType> {
    fn get_stack(&self) -> &Vec<CPUType> {
        &self.stack
    }
    fn get_port(&self, port: usize) -> CPUType {
        self.port[port]
    }
    fn get_accumulator(&self) -> &CPUType {
        &self.accumulator
    }
    fn get_jump_locations(&self) -> &Vec<JumpLocation> {
        &self.jump_locations
    }
}
