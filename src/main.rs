mod error;
mod object;
mod token;
mod scanner;
mod expr;
mod astprinter;
mod parser;
mod interpreter;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use error::LoxError;
use std::{io::{self, BufRead, Write}, env, fs};

use crate::astprinter::AstPrinter;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt().expect("Cannot run prompt."),
        2 => run_file(&args[1]).expect("Cannot run file"),
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64);
        }
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let bytes = fs::read_to_string(path)?;
    if run(bytes).is_err() {
        std::process::exit(65);
    }
    Ok(())     
}

fn run_prompt() -> io::Result<()>{
    let mut reader = io::stdin().lock().lines();
    loop {
        print!("> ");
        io::stdout().flush()?; 
        
        match reader.next(){
            None => break,
            Some(line) => match line {
                Err(e) => return Err(e),
                Ok(line) => if run(line).is_err() {},
            }
        } 
    }  
    Ok(()) 
}


fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    let mut parser = Parser::new(tokens.to_owned());
    let expression = match parser.parse() {
        Ok(expr) => expr,
        Err(_) => return Ok(()),
    };

    let interp = Interpreter {};
    interp.interpret(&expression);

    // let printer = AstPrinter {};
    // println!("{}", printer.print(&expression)?);
    Ok(())
}

