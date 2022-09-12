use colored::Colorize;
use wasm_bindgen::prelude::*;

pub mod getter;
pub mod jump_location;
pub mod main;
pub mod opcodes;
pub mod run;
pub mod show;

pub type CPUType = usize;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

#[wasm_bindgen(module = "/functions.js")]
extern "C" {
    fn out(s: &str);
}

pub enum PrintT {
    Error,
    Info,
    Lexer,
    Cpu,
    Syntax,
    Clear,
}

pub fn printx(type_: PrintT, message: &str) {
    let prefix = match type_ {
        PrintT::Error => format!("[Error]: ").red(),
        PrintT::Info => format!("[Info]: ").green(),
        PrintT::Lexer => format!("[Lexer]: ").blue(),
        PrintT::Cpu => format!("[Cpu]: ").yellow(),
        PrintT::Syntax => format!("[Syntax]: ").yellow(),
        PrintT::Clear => "".to_string().white(),
    };
    println!("{}{}", prefix, message);
    //out(&format!("{}{}", prefix, message));
}

#[macro_export]
macro_rules! log {
    (Error, $($str:tt),*) => {
        use crate::cpu::{printx, PrintT};
        printx(PrintT::Error, $($str),*);
    };
    (Error, f($($format:tt),*)) => {
        printx(PrintT::Error, format!($($format),*).as_str());
    };
    (Info, $($str:tt),*) => {
        printx(PrintT::Info, $($str),*);
    };
    (Info, f($($format:tt),*)) => {
        printx(PrintT::Info, format!($($format),*).as_str());
    };
    (Lexer, $($str:tt),*) => {
        use crate::cpu::{printx, PrintT};
        printx(PrintT::Lexer, $($str),*);
    };
    (Lexer, f($($format:tt),*)) => {
        use crate::cpu::{printx, PrintT};
        printx(PrintT::Lexer, format!($($format),*).as_str());
    };
    (Cpu, $($str:tt),*) => {
        printx(PrintT::Cpu, $($str),*);
    };
    (Cpu, f($($format:tt),*)) => {
        printx(PrintT::Cpu, format!($($format),*).as_str());
    };
    (Syntax, $($str:tt),*) => {
        printx(PrintT::Syntax, $($str),*);
    };
    (Syntax, f($($format:tt),*)) => {
        printx(PrintT::Syntax, format!($($format),*).as_str());
    };
}
