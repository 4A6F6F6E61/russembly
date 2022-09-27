#![allow(dead_code)]
use {colored::Colorize, std::cell::RefCell, wasm_bindgen::prelude::*};

pub mod display;
pub mod main;

pub type CPUType = usize;

thread_local! {
    pub static GLOBAL_OUTPUT: RefCell<String> =  RefCell::new(String::from(""));
    pub static CPU_ERROR_COUNT: RefCell<usize> =  RefCell::new(0usize);
    pub static LEXER_ERROR_COUNT: RefCell<usize> =  RefCell::new(0usize);
    pub static WASM: RefCell<bool> = RefCell::new(true)
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

pub fn set_wasm(b: bool) {
    WASM.with(|x| {
        *x.borrow_mut() = b;
    });
}
pub fn wasm() -> bool {
    let mut y: bool = true;
    WASM.with(|x| {
        y = *x.borrow();
    });
    y
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

#[derive(Debug, Clone, PartialEq)]
pub struct JumpLocation {
    pub name: String,
    pub line: usize,
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
    if wasm() {
        let prefix = match type_ {
            PrintT::Error => "<span class=\"error\">[Error]:</span> ",
            PrintT::Info => "<span class=\"info\">[Info]:</span> ",
            PrintT::Lexer => "<span class=\"lexer\">[Lexer]:</span> ",
            PrintT::Cpu => "<span class=\"cpu\">[CPU]:</span> ",
            PrintT::Syntax => "<span class=\"syntax\">[Syntax]:</span> ",
            PrintT::Clear => "",
        };
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
    } else {
        let prefix = match type_ {
            PrintT::Error => format!("[Error]: ").red(),
            PrintT::Info => format!("[Info]: ").green(),
            PrintT::Lexer => format!("[Lexer]: ").blue(),
            PrintT::Cpu => format!("[Cpu]: ").yellow(),
            PrintT::Syntax => format!("[Syntax]: ").yellow(),
            PrintT::Clear => "".to_string().white(),
        };
        match type_ {
            PrintT::Clear => {
                print!("{}{}", prefix, message);
            }
            _ => {
                print!("{}{}\n", prefix, message);
            }
        };
    }
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
        printx(PrintT::Info, $($str),*);
    };
    (Info, f($($format:tt),*)) => {
        use crate::cpu::{printx, PrintT};
        printx(PrintT::Info, format!($($format),*).as_str());
    };
    (Lexer, $($str:tt),*) => {
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
    (Clear, $($str:tt),*) => {
        printx(PrintT::Clear, $($str),*);
    };
    (Clear, f($($format:tt),*)) => {
        printx(PrintT::Clear, format!($($format),*).as_str());
    };
}

#[derive(Debug, Clone)]
pub enum Var<CPUType> {
    String(StringVar),
    Number(NumberVar<CPUType>),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StringVar {
    pub name: String,
    pub value: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NumberVar<CPUType> {
    pub name: String,
    pub value: CPUType,
}
