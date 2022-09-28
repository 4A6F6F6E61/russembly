#![allow(dead_code)]
use {
    crate::{
        cpu::{
            cpu_error, printx, CPUType, JumpLocation, NumberVar, PrintT, StringVar, Var,
            CPU_ERROR_COUNT, LEXER_ERROR_COUNT,
        },
        lexer::{Function, Lexer, Line, Token, TokenType},
        log,
    },
    colored::{ColoredString, Colorize},
    conv::prelude::*,
    std::{
        fmt::Debug,
        fs::File,
        io::{self, BufRead},
        iter::Peekable,
        num::ParseIntError,
        path::Path,
        slice::Iter,
    },
};

#[derive(Debug, Clone)]
pub struct CPU<CPUType> {
    pub stack: Vec<CPUType>,
    pub port: [CPUType; 8],
    pub vars: Vec<Var<CPUType>>,
    pub accumulator: CPUType,
    pub jump_locations: Vec<JumpLocation>,
    pub error_count: usize,
    pub functions: Vec<Function>,
}

impl CPU<CPUType> {
    pub fn new<'t>() -> Result<Self, &'t str> {
        Ok(CPU {
            stack: vec![],
            port: [0; 8],
            vars: vec![],
            accumulator: 0,
            jump_locations: vec![],
            error_count: 0,
            functions: vec![],
        })
    }
    /*
     * Getter
     * -------------------------------------------------------
     */
    pub fn get_stack(&self) -> &Vec<CPUType> {
        &self.stack
    }
    pub fn get_port(&self, port: usize) -> CPUType {
        self.port[port]
    }
    pub fn get_accumulator(&self) -> &CPUType {
        &self.accumulator
    }
    pub fn get_jump_locations(&self) -> &Vec<JumpLocation> {
        &self.jump_locations
    }
    // -------------------------------------------------------
    /*
     * -------------------------------------------------
     * Only for Debugging
     */
    pub fn push_to_stack(&mut self, value: CPUType) {
        self.stack.push(value);
    }
    pub fn pop_from_stack(&mut self) -> Option<usize> {
        self.stack.pop()
    }
    /*
     * -------------------------------------------------
     */
    pub fn add_jump_location(&mut self, name: String, line: usize) {
        self.jump_locations.push(JumpLocation { name, line })
    }
    /*
     * Load a file and get the Tokens from the Lexer
     */
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_file(&mut self, path: &str) -> Option<()> {
        self.functions = vec![];
        let mut lexer = Lexer::new();
        lexer.setup_pb();
        let mut line_count = 0;

        if let Ok(file) = self.read_lines(path) {
            line_count = file.count();
        }
        if let Ok(lines) = self.read_lines(path) {
            log!(Lexer, "Parsing tokens...");
            lines.for_each(|line| {
                let l = line.unwrap();
                lexer.run(l, line_count);
            });
            lexer.finish_pb();
            let mut lexer_error_c = 0usize;
            LEXER_ERROR_COUNT.with(|count| {
                lexer_error_c = *count.borrow();
            });
            log!(
                Lexer,
                f("Parsing the tokens returned {} errors", lexer_error_c)
            );
            if lexer_error_c != 0 {
                //exit(1)
            }
            log!(Info, "Finished parsing tokens");
        } else {
            log!(Error, "Unable to read lines");
        }
        //lexer.show_lines();
        //if let Some(functions) = lexer.get_functions() {
        //    println!("{:#?}", functions);
        //}
        //lexer.get_lines()
        lexer.generate_functions();
        if let Some(f) = lexer.get_functions() {
            self.functions = f;
            return Some(());
        }
        None
    }
    /*
     * Load a string and get the Tokens from the Lexer
     */
    pub fn load_string(&mut self, string: &str) -> Option<()> {
        self.functions = vec![];
        let mut lexer = Lexer::new();
        //lexer.setup_pb(); // this is not supported on wasm
        let code = &string.replace("~", "\n");
        let line_count = code.lines().count();

        if line_count != 0 {
            code.lines().for_each(|line| {
                let l = line.to_string();
                lexer.run(l, line_count);
            });
            //lexer.finish_pb();
            let mut lexer_error_c = 0usize;
            LEXER_ERROR_COUNT.with(|count| {
                lexer_error_c = *count.borrow();
            });
            log!(
                Lexer,
                f("Parsing the tokens returned {} errors", lexer_error_c)
            );
            if lexer_error_c != 0 {
                //exit(1)
            }
            log!(Info, "Finished parsing tokens");
        } else {
            log!(Error, "Please provide some Code");
        }
        lexer.generate_functions();
        if let Some(f) = lexer.get_functions() {
            self.functions = f;
            return Some(());
        }
        None
    }

    /*
     * Read lines function I copied from Stackoverflow
     */
    fn read_lines<P>(&mut self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    /*
     *
     */
    pub fn get_port_from_str(&mut self, port_str: String) -> Result<usize, ParseIntError> {
        let mut chars = port_str.chars();
        chars.next();
        chars.as_str().parse::<usize>()
    }

    /*
     * Function for getting the stringified JSON representation
     * for this CPU struct
     */
    pub fn get_json(&self) -> String {
        let mut output = String::new();
        output.push_str("{");
        //stack: vec![],
        output = format!("{}\"stack\":{:?},", output, self.stack);
        //port: [0; 8],
        output = format!("{}\"ports\":{{", output);
        self.port.iter().enumerate().for_each(|(i, port)| {
            if i == 0 {
                output = format!("{}\"{}\":\"{}\"", output, i, port);
            } else {
                output = format!("{},\"{}\":\"{}\"", output, i, port);
            }
        });
        output = format!("{}}},", output);
        //vars: vec![],
        output = format!("{}\"vars\":[", output,);
        self.vars.iter().enumerate().for_each(|(i, v)| {
            match v {
                Var::String(x) => {
                    if i == 0 {
                        output = format!(
                            "{}{{\"name\":{},\"value\":\"{}\"}}",
                            output, x.name, x.value
                        );
                    } else {
                        output = format!(
                            "{},{{\"name\":{},\"value\":\"{}\"}}",
                            output, x.name, x.value
                        );
                    }
                }
                Var::Number(x) => {
                    if i == 0 {
                        output = format!(
                            "{}{{\"name\":{},\"value\":\"{}\"}}",
                            output, x.name, x.value
                        );
                    } else {
                        output = format!(
                            ",{}{{\"name\":{},\"value\":\"{}\"}}",
                            output, x.name, x.value
                        );
                    }
                }
            }
            //output = format!("{}   {}: {},\n", output);
        });
        output.push_str("],");
        //accumulator: 0,
        output = format!("{}\"accumulator\":\"{}\"", output, self.accumulator);
        //jump_locations: vec![],
        output.push_str("}");
        return output;
    }

    /*
     * Function to find a variable in the current scope
     */
    pub fn try_get_var(&self, var: &str) -> Option<Var<CPUType>> {
        let mut out = Var::Number(NumberVar {
            name: "tmp".to_string(),
            value: 10,
        });
        let mut found: bool = false;
        self.vars.iter().for_each(|v| match v {
            Var::String(x) => {
                if x.name == var {
                    found = true;
                    out = Var::String(StringVar {
                        name: x.name.clone(),
                        value: x.value.clone(),
                    });
                }
            }
            Var::Number(x) => {
                if x.name == var {
                    found = true;
                    out = Var::Number(NumberVar {
                        name: x.name.clone(),
                        value: x.value.clone(),
                    });
                }
            }
        });
        if found {
            Some(out)
        } else {
            None
        }
    }

    /*
     * Function for generating a pretty error message
     */
    fn cpu_line_error(
        &self,
        error: &str,
        line_string: String,
        line_number: usize,
        error_loc: usize,
        error_line: &str,
    ) {
        let line_len = format!("{}", line_number).len();
        let mut space = "".to_string();
        for _ in 0..=line_len {
            space.push(' ');
        }
        log!(Error, f("{}", error));
        let blue_line: ColoredString;
        if cfg!(target_arch = "wasm32") {
            blue_line = "<span class=\"blue\">|</span>".blue(); // the .blue() is ignored by the html and is only here
                                                                // to ensure that the type is ColoredString
        } else {
            blue_line = "|".blue();
        }
        let blue_line_number: ColoredString;
        if cfg!(target_arch = "wasm32") {
            blue_line_number = format!("<span class=\"blue\">{}</span>", line_number).blue();
            // the .blue() is ignored by the html and is only here
            // to ensure that the type is ColoredString
        } else {
            blue_line_number = format!("{}", line_number).blue();
        }
        log!(Clear, f("{}{}\n", space, blue_line));
        log!(
            Clear,
            f("{} {} {}\n", blue_line_number, blue_line, line_string)
        );
        let mut temp = "".to_string();
        let mut arrows = "".to_string();
        temp.push_str(&format!("{}  ", blue_line));
        let strings: Vec<&str> = line_string.split(" ").collect();
        for i in 0..strings.len() {
            if i == error_loc {
                for _ in strings[i].chars() {
                    arrows.push('^');
                }
            } else {
                for _ in strings[i].chars() {
                    temp.push(' ');
                }
            }
        }
        let el_red = error_line.red();
        let arrows_red = arrows.red();
        log!(Clear, f("{}{}{} {}\n", space, temp, arrows_red, el_red));
        log!(Clear, f("{}{}\n", space, blue_line));
    }
    /*
     * --------------------------------------------------------------
     * Functions for the Interpreter
     * --------------------------------------------------------------
     */

    pub fn run_main(&mut self) {
        log!(Clear, "\nOutput:\n");
        log!(Clear, "-------------------------\n");
        self.run_function("main", "");
        log!(Clear, "-------------------------\n");
        let mut error_count = 0usize;
        CPU_ERROR_COUNT.with(|count| {
            error_count = *count.borrow();
        });
        log!(
            Cpu,
            f("Interpreting the tokens returned {} errors", error_count)
        );
    }

    fn run_function(&mut self, name: &str, _arguments: &str) {
        for f in self.functions.clone() {
            if name == f.name {
                self.run_lines(f.lines);
                // clear variables
                self.vars = vec![];
                return;
            }
        }
        log!(Error, f("function `{name}` not found"));
    }

    fn run_lines(&mut self, lines: Vec<Line>) {
        for i in 0..(lines.len()) {
            let mut token_iter = lines[i].tokens.iter().peekable();
            while token_iter.peek().is_some() {
                let token = token_iter.next().unwrap();
                match &token.token_type {
                    TokenType::OpCode => {
                        self.run_opcodes(&mut token_iter, token, lines[i].clone().as_string, i)
                    }
                    TokenType::JumpLocation(_jump_location) => {}
                    TokenType::Bracket => {}
                    TokenType::Keyword => {
                        self.run_keywords(&mut token_iter, token, lines[i].clone().as_string, i)
                    }
                    TokenType::String => {}
                    TokenType::Comment => {}
                    // Prints a new Line
                    TokenType::NewLine => {
                        log!(Clear, "\n");
                    }
                    _ => {
                        printx(
                            PrintT::Error,
                            format!("unexpected token '{}' at line {i}", token.value).as_str(),
                        );
                    }
                }
            }
        }
    }

    pub fn run_keywords(
        &mut self,
        token_iter: &mut Peekable<Iter<Token>>,
        token: &Token,
        _line: String,
        _line_number: usize,
    ) {
        match token.value.as_str() {
            // "fn" => {
            //     token_iter.next();
            //     token_iter.next();
            // }
            "let" => {
                let nt = match token_iter.next() {
                    Some(x) => x,
                    None => {
                        cpu_error();
                        log!(Error, "Expected Arguments after let");
                        return;
                    }
                };
                let var_name = match nt.token_type {
                    TokenType::VarName => nt.value.as_str(),
                    _ => {
                        log!(Error, "Expected variable name");
                        cpu_error();
                        return;
                    }
                };
                if let Some(nt) = token_iter.next() {
                    match nt.token_type {
                        TokenType::Comma => { /* Do nothing, just for checking*/ }
                        _ => {
                            cpu_error();
                            log!(Error, "Expected Comma");
                        }
                    }
                } else {
                    cpu_error();
                    log!(Error, "Expected Comma");
                }
                if let Some(nt) = token_iter.next() {
                    match nt.token_type {
                        TokenType::String => {
                            self.vars.push(Var::String(StringVar {
                                name: var_name.to_string(),
                                value: nt.value.clone(),
                            }));
                        }
                        TokenType::Number(x) => {
                            self.vars.push(Var::Number(NumberVar {
                                name: var_name.to_string(),
                                value: x,
                            }));
                        }
                        _ => {
                            printx(
                                PrintT::Error,
                                "You can only store Strings and Numbers inside a Variable",
                            );
                            cpu_error();
                        }
                    }
                } else {
                    cpu_error();
                    log!(Error, "Expected value for let");
                }
            }
            _ => {}
        }
    }

    pub fn run_opcodes(
        &mut self,
        token_iter: &mut Peekable<Iter<Token>>,
        token: &Token,
        line: String,
        line_number: usize,
    ) {
        match token.value.as_str() {
            "push" => {
                if let Some(nt) = token_iter.next() {
                    match nt.token_type {
                        TokenType::Number(x) => self.stack.push(x),
                        _ => {
                            cpu_error();
                            log!(Error, "You can only push Numbers to the Stack!");
                        }
                    }
                } else {
                    cpu_error();
                    log!(Error, "Expected Number after push");
                }
            }
            "pop" => {
                self.stack.pop();
            }
            // move value
            "mov" => {
                if let (Some(port_or_accu), Some(comma), Some(value)) =
                    (token_iter.next(), token_iter.next(), token_iter.next())
                {
                    match comma.token_type {
                        // check for comma
                        TokenType::Comma => {}
                        _ => {
                            cpu_error();
                            log!(Error, "Expected Comma");
                        }
                    }
                    match port_or_accu.token_type {
                        // push to port
                        TokenType::Port => {
                            if let Ok(number) = self.get_port_from_str(port_or_accu.value.clone()) {
                                self.port[number] = match value.token_type {
                                    TokenType::Number(x) => x,
                                    TokenType::Port => {
                                        let port =
                                            self.get_port_from_str(value.value.clone()).unwrap(); // get the Port number
                                        self.port[port]
                                    }
                                    _ => {
                                        cpu_error();
                                        log!(Error,"You can only move a number or the value of a Port to this Port");
                                        return;
                                    }
                                }
                            }
                        }
                        // push to accumulator
                        TokenType::Accumulator => {
                            self.accumulator = match value.token_type {
                                TokenType::Number(x) => x,
                                TokenType::Port => {
                                    let port = self.get_port_from_str(value.value.clone()).unwrap(); // get the Port number
                                    self.port[port]
                                }
                                _ => {
                                    cpu_error();
                                    log!(Error,"You can only move a number or the value of a Port to the Accumulator");
                                    return;
                                }
                            }
                        }
                        _ => {
                            cpu_error();
                            log!(Error, "Expected Port or Accu!");
                        }
                    }
                } else {
                    cpu_error();
                    log!(Error, "Expected more Tokens after mov");
                    log!(Syntax, "mov <Port or Accu> <,> <value>");
                }
            }
            // add top 2 number from stack together and push them on the stack
            "adds" => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(a + b);
            }
            // sub top 2 number from stack together and push them on the stack
            "subs" => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(a - b);
            }
            // mul top 2 number from stack together and push them on the stack
            "muls" => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(a * b);
            }
            // div top 2 number from stack together and push them on the stack
            "divs" => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(a / b);
            }
            "djnz" => {}
            "jmp" => {}
            "setb" => {}
            "end" => {}
            // print given string or number
            "prnt" => {
                if let Some(nt) = token_iter.next() {
                    match nt.token_type {
                        TokenType::String => {
                            let v = &nt.value;
                            printx(PrintT::Clear, &format!("{}", v));
                        }
                        TokenType::Number(x) => {
                            printx(PrintT::Clear, &format!("{}", x));
                        }
                        TokenType::Accumulator => {
                            printx(PrintT::Clear, &format!("{}", self.accumulator));
                        }
                        TokenType::Stack => {
                            printx(PrintT::Clear, &format!("{:?}", self.stack));
                        }
                        TokenType::Port => {
                            if let Ok(port) = self.get_port_from_str(nt.value.clone()) {
                                printx(PrintT::Clear, &format!("{}", self.port[port]));
                            } else {
                                cpu_error();
                                log!(Error, "Invalid Port");
                            }
                        }
                        _ => {
                            if let Some(var) = self.try_get_var(&nt.value) {
                                match var {
                                    Var::Number(x) => {
                                        printx(PrintT::Clear, &format!("{}", x.value));
                                    }
                                    Var::String(x) => {
                                        printx(PrintT::Clear, &format!("{}", x.value));
                                    }
                                }
                            } else {
                                cpu_error();
                                let value = nt.value.clone();
                                //log!(Error, f("Unable to find variable \"{}\"", value));
                                self.cpu_line_error(
                                    &format!("cannot find value `{value}` in this scope"),
                                    line,
                                    line_number,
                                    1,
                                    "not found in this scope",
                                );
                            }
                        }
                    }
                } else {
                    cpu_error();
                    self.cpu_line_error(
                        "expected token after prnt statement",
                        line,
                        line_number,
                        1,
                        "",
                    );
                }
            }
            "call" => {
                if let Some(funcion_name) = token_iter.next() {
                    self.run_function(&funcion_name.value, "");
                } else {
                    log!(Error, "Expected function name after call statement");
                }
            }
            &_ => {}
        }
    }
    //--------------------------------------------------------------
    /*
     * Opcodes for Debugging and testing:
     * -------------------------------------------------------------
     */
    pub fn mov<T>(&mut self, port: usize, value: T)
    where
        CPUType: ValueFrom<T>,
    {
        self.port[port] = value.value_as::<CPUType>().unwrap();
    }
    pub fn mova<T>(&mut self, value: T)
    where
        CPUType: ValueFrom<T>,
    {
        self.accumulator = value.value_as::<CPUType>().unwrap();
    }
    pub fn mova_p(&mut self, port: usize) {
        self.accumulator = self.port[port];
    }
    pub fn add(&mut self) -> () {
        let a = self.pop_from_stack().unwrap();
        let b = self.pop_from_stack().unwrap();
        self.push_to_stack(a + b);
    }
    pub fn sub(&mut self) -> () {
        let a = self.pop_from_stack().unwrap();
        let b = self.pop_from_stack().unwrap();
        self.push_to_stack(a - b);
    }
    pub fn mul(&mut self) -> () {
        let a = self.pop_from_stack().unwrap();
        let b = self.pop_from_stack().unwrap();
        self.push_to_stack(a * b);
    }
    pub fn div(&mut self) -> () {
        let a = self.pop_from_stack().unwrap();
        let b = self.pop_from_stack().unwrap();
        self.push_to_stack(a / b);
    }
    pub fn addp(&mut self, port: usize) {
        self.accumulator += self.port[port];
    }
    pub fn subp(&mut self, port: usize) {
        self.accumulator -= self.port[port];
    }
    pub fn djnz(&mut self, port: usize, jmp_loc_name: String) {
        if self.port[port] != 0 {
            self.port[port] -= 1;
            self.jmp(jmp_loc_name);
        }
    }
    pub fn jmp(&mut self, _jmp_loc_name: String) {
        /* TODO */
    }
    pub fn setb(&mut self, port_bit: String) {
        let s = port_bit.split("^");
        let vec = s.collect::<Vec<&str>>();
        let mut chars = vec[0].chars();
        chars.next();
        match (chars.as_str().parse::<usize>(), vec[1].parse::<usize>()) {
            (Ok(port), Ok(bit)) => {
                if port >= 7 {
                    println!("Port: {} out of bounds (0 - 7)", port);
                    return;
                }
                if bit > 63 {
                    println!(
                        "Setting the {}th bit will lead to a stack overflow (max is 63)",
                        bit
                    );
                    return;
                }
                self.port[port] |= 1 << bit;
            }
            _ => println!("Error parsing: {}", port_bit),
        }
    }
    // -------------------------------------------------------------
}
