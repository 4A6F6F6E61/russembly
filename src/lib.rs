mod cpu;
mod lexer;

use crate::cpu::main::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run_rusm(code: &str) -> String {
    let mut cpu = match CPU::new() {
        Ok(cpu) => cpu,
        Err(err) => {
            log(&format!("{}", err));
            return "".to_string();
        }
    };

    if let Some(tokens) = cpu.load_string(code) {
        cpu.run_tokens(tokens);
    }
    return cpu.output.concat();
}
