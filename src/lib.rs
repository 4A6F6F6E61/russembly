mod cpu;
mod lexer;

use crate::{
    cpu::get_global_output,
    cpu::{main::*, printx, PrintT},
    lexer::Lexer,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_rusm(code: &str) -> String {
    let mut cpu = match CPU::new() {
        Ok(cpu) => cpu,
        Err(_) => {
            //log(&format!("{}", err));
            return "".to_string();
        }
    };
    if let Some(lines) = cpu.load_string(code) {
        cpu.run_lines(lines);
    }
    return get_global_output();
}

#[wasm_bindgen]
pub fn wasm_parse_line(lexer: *mut Lexer) {
    let t = unsafe { &*lexer };
    printx(PrintT::Clear, &format!("{:?}", t));
}

#[wasm_bindgen]
pub fn wasm_new_lexer() -> *mut Lexer {
    &mut Lexer::new()
}
