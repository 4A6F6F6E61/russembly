use {colored::Colorize, std::cell::RefCell, wasm_bindgen::prelude::*};

pub mod getter;
pub mod jump_location;
pub mod main;
pub mod opcodes;
pub mod run;
pub mod show;

pub type CPUType = usize;

thread_local! {
    pub static GLOBAL_OUTPUT: RefCell<String> =  RefCell::new(String::from(""));
    pub static CPU_ERROR_COUNT: RefCell<usize> =  RefCell::new(0usize);
    pub static LEXER_ERROR_COUNT: RefCell<usize> =  RefCell::new(0usize);
}

pub fn cpu_error() {
    CPU_ERROR_COUNT.with(|count| {
        *count.borrow_mut() += 1;
    });
}
pub fn lexer_error() {
    LEXER_ERROR_COUNT.with(|count| {
        *count.borrow_mut() += 1;
    });
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
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
    match type_ {
        PrintT::Clear => {
            GLOBAL_OUTPUT.with(|output| {
                *output.borrow_mut() = format!("{}{}{}", *output.borrow(), prefix, message);
            });
        }
        _ => {
            GLOBAL_OUTPUT.with(|output| {
                *output.borrow_mut() = format!("{}{}{}\n", *output.borrow(), prefix, message);
            });
        }
    };
    //out(&format!("{}{}", prefix, message));
}

#[allow(dead_code)]
pub fn get_global_output() -> String {
    let mut output = String::new();
    GLOBAL_OUTPUT.with(|text| {
        output = text.borrow().clone();
    });
    output
}

#[macro_export]
macro_rules! log {
    (Error, $($str:tt),*) => {
        use crate::cpu::{printx, PrintT};
        printx(PrintT::Error, $($str),*);
    };
    (Error, f($($format:tt),*)) => {
        use crate::cpu::{printx, PrintT};
        printx(PrintT::Error, format!($($format),*).as_str());
    };
    (Info, $($str:tt),*) => {
        use crate::cpu::{printx, PrintT};
        printx(PrintT::Info, $($str),*);
    };
    (Info, f($($format:tt),*)) => {
        use crate::cpu::{printx, PrintT};
        printx(PrintT::Info, format!($($format),*).as_str());
    };
    (Lexer, $($str:tt),*) => {
        use crate::cpu::{printx, PrintT};
        printx(PrintT::Lexer, $($str),*);
    };
    (Lexer, f($($format:tt),*)) => {
        printx(PrintT::Lexer, format!($($format),*).as_str());
    };
    (Cpu, $($str:tt),*) => {
        use crate::cpu::{printx, PrintT};
        printx(PrintT::Cpu, $($str),*);
    };
    (Cpu, f($($format:tt),*)) => {
        use crate::cpu::{printx, PrintT};
        printx(PrintT::Cpu, format!($($format),*).as_str());
    };
    (Syntax, $($str:tt),*) => {
        printx(PrintT::Syntax, $($str),*);
    };
    (Syntax, f($($format:tt),*)) => {
        use crate::cpu::{printx, PrintT};
        printx(PrintT::Syntax, format!($($format),*).as_str());
    };
}
