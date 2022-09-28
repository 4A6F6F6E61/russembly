mod cpu;
mod lexer;
mod test;

use crate::cpu::main::*;

fn main() {
    let mut cpu = match CPU::new() {
        Ok(cpu) => cpu,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    if let Some(()) = cpu.load_file("./src/testing.rusm") {
        cpu.mov(0, 19);
        //cpu.run_lines(lines);
        cpu.run_main();
        println!("{}", cpu.get_json());
    }

    //cpu.setb("P1^64".to_string());

    //cpu.pop_from_stack();

    /*for i in 0..usize::BITS {
        cpu.setb(format!("P0^{}", i));
    }*/
    //cpu.mova(8);
    //cpu.mova(90);
    //cpu.addp(0);
    //cpu.push_to_stack(10);
    //cpu.djnz(0, "test".to_string());
    //cpu.add_jump_location("idk".to_string(), 10);
    //cpu.add_jump_location("test".to_string(), 50);
    println!("{}", cpu)
}
