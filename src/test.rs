use crate::cpu::{OpCodes, CPU, CpuGetter};

#[test]
#[cfg(test)]
fn add() -> () {
    let mut cpu = CPU::<i64, f64>::new();
    // Integer Register
    cpu.mov(0, 10);
    cpu.mov(1, 8);
    cpu.addi_r(0, 1);
    // Floating Point Register
    cpu.mov_f(0, 10.5);
    cpu.mov_f(1, 7.5);
    cpu.addf_r(0, 1);
    // Stack
    cpu.push_to_stack(10);
    cpu.push_to_stack(8);
    cpu.add();

    assert_eq!(cpu.get_register()[0], 18);
    assert_eq!(cpu.get_floating_point_register()[0], 18.0);
    assert_eq!(cpu.pop_from_stack(), 18);
}

#[test]
#[cfg(test)]
fn minus() -> () {
    let mut cpu = CPU::<i64, f64>::new();
    cpu.mov(0, 10);
    cpu.mov(1, 8);
    // not implemented yet
    cpu.subi_r(0, 1);
    assert_eq!(cpu.get_register()[0], 18);
}