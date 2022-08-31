use colored::Colorize;

pub mod getter;
pub mod jump_location;
pub mod main;
pub mod opcodes;
pub mod show;

pub type CPUType = usize;

pub enum PrintT {
    Error,
    Info,
    Lexer,
    Cpu,
}

pub fn printx(type_: PrintT, message: &str) {
    let prefix = match type_ {
        PrintT::Error => format!("[Error]:").red(),
        PrintT::Info => format!("[Info]:").green(),
        PrintT::Lexer => format!("[Lexer]:").blue(),
        PrintT::Cpu => format!("[Cpu]:").yellow(),
    };
    println!("{} {}", prefix, message);
}

#[macro_export]
macro_rules! log {
    (Error, $($str:tt),*) => {
        use crate::cpu::{printx, PrintT};
        printx(PrintT::Error, $($str),*);
    };
    (Info, $($str:tt),*) => {
        printx(PrintT::Info, $($str),*);
    };
    (Error, f($($format:tt),*)) => {
        use crate::cpu::{printx, PrintT};
        printx(PrintT::Error, format!($($format),*).as_str());
    };
    (Info, f($($format:tt),*)) => {
        printx(PrintT::Info, format!($($format),*).as_str());
    };
    (Lexer, $($str:tt),*) => {
        use crate::cpu::{printx, PrintT};
        printx(PrintT::Lexer, $($str),*);
    };
    (Cpu, $($str:tt),*) => {
        printx(PrintT::Cpu, $($str),*);
    };
    (Lexer, f($($format:tt),*)) => {
        use crate::cpu::{printx, PrintT};
        printx(PrintT::Lexer, format!($($format),*).as_str());
    };
    (Cpu, f($($format:tt),*)) => {
        printx(PrintT::Cpu, format!($($format),*).as_str());
    };
}
/*
printx(
    PrintT::Info,
    format!("Lexer returned {} errors", lexer_error_c).as_str(),
); */
