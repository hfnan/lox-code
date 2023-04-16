mod error;
mod tokentype;
mod token;
mod scanner;

use scanner::Scanner;
use error::LoxError;
use std::{io::{self, BufRead, Write}, env::args, fs::read_to_string};
fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]).expect("Cannot run file");
    } else {
        run_prompt().expect("Cannot run prompt");
    }
}

fn run_file(path: &String) -> io::Result<()> {
    let bytes = read_to_string(path)?;
    if let Err(e) = run(bytes) {
        e.report("");
        std::process::exit(65);
    }
    Ok(())    
}

fn run_prompt() -> io::Result<()>{
    let stdin = io::stdin();
    let mut reader =  stdin.lock().lines();
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        let line = reader.next();

        match line {
            None => break,
            Some(line) => match line {
                Err(e) => return Err(e),
                Ok(line) => if let Err(e) = run(line) { e.report(""); }
            }
        } 
    }  
    Ok(()) 
}



fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

