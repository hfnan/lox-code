mod error;
mod object;
mod token;
mod scanner;
mod expr;
mod parser;
mod interpreter;
mod stmt;
mod environment;

use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use error::LoxError;

use std::{io::{self, BufRead, Write}, env, fs};

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
    let mut interp = Interpreter::new();
    if run(bytes, &mut interp).is_err() {
        std::process::exit(65);
    }
    Ok(())     
}

fn run_prompt() -> io::Result<()>{
    let mut reader = io::stdin().lock().lines();
    let mut interp = Interpreter::new();
    loop {
        print!("> ");
        io::stdout().flush()?; 
        
        match reader.next(){
            None => break,
            Some(line) => match line {
                Err(e) => return Err(e),
                Ok(line) => if run(line, &mut interp).is_err() {},
            }
        } 
    }  
    Ok(()) 
}


fn run(source: String, interp: &mut Interpreter) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    // for token in tokens {
    //     println!("{}", token);
    // }
    
    let mut parser = Parser::new(tokens.to_owned());
    let statements = match parser.parse() {
        Ok(expr) => expr,
        Err(_) => return Ok(()),
    };

    interp.interpret(statements);

    // let printer = AstPrinter {};
    // println!("{}", printer.print(&expression)?);
    Ok(())
}

