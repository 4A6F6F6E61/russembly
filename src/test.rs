#[cfg(test)]
use crate::cpu::{CpuGetter, OpCodes, CPU};

#[test]
fn add() -> () {
    let mut cpu = CPU::<f64>::new();
    // Integer Register
    cpu.mov(0, 10);
    cpu.mov(1, 8);
    cpu.addi_p(0, 1);
    // Stack
    cpu.push_to_stack(10.0);
    cpu.push_to_stack(8.0);
    cpu.add();

    assert_eq!(cpu.get_port()[0], 18.0);
    assert_eq!(cpu.pop_from_stack(), 18);
}

#[test]
#[cfg(test)]
fn minus() -> () {
    let mut cpu = CPU::<f64>::new();
    cpu.mov(0, 10.0);
    cpu.mov(1, 8.0);
    // not implemented yet
    cpu.subi_p(0, 1);
    assert_eq!(cpu.get_register()[0], 18);
}
