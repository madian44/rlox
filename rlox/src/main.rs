use std::{env, fs, io, io::Write, process};
use rlox::InterpretResult;

fn main() {
    println!("Hello, Rlox!");

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: {} [script]", args[0]);
            process::exit(64);
        }
    }
}

fn run_prompt() {
    let reporter = rlox::DefaultReporter::default();
    let mut vm = rlox::Vm::new(&reporter);
    loop {
        print!("(lox)> ");
        if io::stdout().flush().is_err() {
            return;
        }
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Err(_) => break,
            Ok(_) => {
                let trimmed_line = line.trim();
                if trimmed_line.is_empty() {
                    break;
                }
                vm.interpret(trimmed_line);
            }
        }
    }
    println!("done");
}

fn run_file(filepath: &str) {
    let contents = fs::read_to_string(filepath);
    if let Err(e) = contents {
        eprintln!("{e}");
        process::exit(74);
    }
    let reporter = rlox::DefaultReporter::default();
    let mut vm = rlox::Vm::new(&reporter);
    match vm.interpret(&contents.unwrap()) {
        InterpretResult::Ok => process::exit(0),
        InterpretResult::CompileError => process::exit(65),
        InterpretResult::RuntimeError => process::exit(70),
    }
}
