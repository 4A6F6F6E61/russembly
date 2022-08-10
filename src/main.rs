mod cpu;
mod lexer;

fn main() {
    use crate::cpu::*;
    //use crate::lexer::*;
    //let mut lexer = Lexer::new("".to_string());
    type CpuInt = i64;
    type CpuFloat = f64;
    let mut cpu = CPU::<i64, f64>::new();

    cpu.mov(0, 999);
    cpu.mov(5, 8);
    cpu.addi_r(0, 5);
    cpu.push_to_stack(10);
    cpu.djnz(0, "test".to_string());
    cpu.add_jump_location("idk".to_string(), 10);
    cpu.add_jump_location("test".to_string(), 50);
    cpu.show_cpu();
}
