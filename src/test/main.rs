#![allow(unused_macros)]
#[cfg(test)]
use crate::cpu::main::*;

macro_rules! new {
    (let $name:ident = new $type:ty;) => {
        let $name = match <$type>::new() {
            Ok(x) => x,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };
    };
    (let mut $name:ident = new $type:ty;) => {
        let mut $name = match <$type>::new() {
            Ok(x) => x,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };
    };
    (
        let $name:ident = new $type:ty;
        $($rest:tt)*
    ) => {
        let $name = match <$type>::new() {
            Ok(x) => x,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };
        new! {
            $($rest)*
        }
    };
    (
        let mut $name:ident = new $type:ty;
        $($rest:tt)*
    ) => {
        let mut $name = match <$type>::new() {
            Ok(x) => x,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };
        new! {
            $($rest)*
        }
    };
}

#[test]
fn add() -> () {
    new! {
        let mut cpu = new CPU<usize>;
        let _cpu2 = new CPU<usize>;
        let mut cpu3 = new CPU<usize>;
    };
    // Integer Register
    cpu.mov(0, 10);
    cpu.mova(8);
    cpu.addp(0);
    assert_eq!(cpu.get_accumulator(), &18);
    // Stack
    cpu.push_to_stack(10);
    cpu.push_to_stack(8);
    cpu.add();
    assert_eq!(cpu.pop_from_stack(), Some(18));

    cpu3.push_to_stack(10);
    println!("{}", cpu3)
}

#[test]
fn minus() -> () {
    new! {
        let mut cpu = new CPU<usize>;
    };
    cpu.mov(0, 10);
    cpu.mova(8);
    // not implemented yet
    cpu.subp(0);
    assert_eq!(cpu.get_port(0), 18);
}

#[test]
fn set_bit() -> () {
    new! {
        let mut cpu = new CPU<usize>;
    };

    cpu.setb("P0^0".to_string());
    assert_eq!(cpu.get_port(0), 0);
    cpu.setb("P0^10".to_string());
    assert_eq!(cpu.get_port(0), 1024)
}

#[test]
fn max_usize() -> () {
    new! {
        let mut cpu = new CPU<usize>;
    };

    for i in 0..usize::BITS {
        cpu.setb(format!("P0^{i}"));
    }
    assert_eq!(cpu.get_port(0), usize::MAX) // 18446744073709551615
}

#[test]
fn lexer_new() -> () {
    use crate::lexer_new::Lexer;
    use std::fs::read_to_string;

    let mut lexer = Lexer::new();
    let code =
        read_to_string("./src/testing.rusm").expect("Should have been able to read the file");
    lexer.parse(code);
    println!("{:#?}", lexer.ast);
}

#[test]
fn split() {
    let test = vec![
        vec!["loop".to_string(), "{".to_string()],
        vec!["prnt".to_string(), "A".to_string()],
        vec!["}".to_string()],
    ];
    let mut split: Vec<String> = vec![];
    for x in test {
        split.push(x.join(" "));
    }
    assert_eq!(split, vec!["loop {", "prnt A", "}"]);
    //let mut braces = vec![];
}
