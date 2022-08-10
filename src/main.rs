mod cpu;
mod lexer;

fn main() {
    use crate::cpu::*;
    //use crate::lexer::*;
    //let mut lexer = Lexer::new("".to_string());
    let mut cpu = CPU::new();

    cpu.mov(0, 999);
    cpu.mov(5, 8);
    cpu.addi_r(0, 5);
    cpu.show_cpu();
}
