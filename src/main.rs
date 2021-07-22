use std::env;
use std::fs;
use std::io;
use std::io::Write;

use rlox::chunk::{Chunk, OpCode};
use rlox::vm::VM;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        eprintln!("Usage: rlox [path]");
    }
}

fn repl() {
    loop {
        let mut line = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        if let Err(_) = io::stdin().read_line(&mut line) {
            println!();
            break;
        }
        if line.len() == 0 {
            println!();
            break;
        }
    }
}

fn run_file(path: &str) {
    let result = fs::read_to_string(path);
    match result {
        Ok(s) => {
            println!("{}", s);
        }
        Err(msg) => {
            eprintln!("{:?}", msg);
        }
    }
}
