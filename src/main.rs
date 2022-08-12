use crate::cpu::OpCodes;

mod cpu;
mod lexer;
mod test;

fn main() {
    use crate::cpu::*;
    //use crate::lexer::*;
    //let mut lexer = Lexer::new("".to_string());
    let mut cpu = CPU::<f64>::new();

    cpu.mov(0, 99.0);
    cpu.mov(1, 8);
    cpu.mova(8.0);
    cpu.mova(90);
    cpu.addp(0);
    cpu.push_to_stack(10.0);
    cpu.djnz(0, "test".to_string());
    cpu.add_jump_location("idk".to_string(), 10);
    cpu.add_jump_location("test".to_string(), 50);
    cpu.show_cpu();
}
