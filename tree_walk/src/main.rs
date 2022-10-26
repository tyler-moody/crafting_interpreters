use clap::Parser;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{stdin, stdout};
use std::io::{Read, Write};
use std::path::PathBuf;

use tree_walk::parse::parse;
use tree_walk::scan::scan_tokens;

#[derive(Parser)]
#[clap(version)]
struct Args {
    /// Lox script to execute
    script: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let result = match args.script {
        None => run_prompt(),
        Some(script) => run_file(script),
    };
    result.unwrap();
}

fn run_file(script: PathBuf) -> std::io::Result<()> {
    let mut file = File::open(script)?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;
    run(source);
    Ok(())
}

fn run_prompt() -> std::io::Result<()> {
    loop {
        print!("> ");
        stdout().flush()?;
        let mut buffer = String::new();
        stdin().read_line(&mut buffer)?;
        if buffer.is_empty() {
            break;
        }
        run(buffer);
    }
    Ok(())
}

fn run(source: String) {
    let tokens = scan_tokens(source);
    if let Ok(ref token) = tokens {
        println!("{:?}", token);
    }
    match tokens {
        Ok(tokens) => {
            let expressions = parse(VecDeque::from_iter(tokens));
            match expressions {
                Ok(expressions) => {
                    for expression in expressions {
                        println!("{:?}", expression);
                    }
                }
                Err(parse_error) => {
                    println!("error parsing: {:?}", parse_error);
                }
            }
        }
        Err(scan_error) => {
            println!("error scanning tokens: {:?}", scan_error)
        }
    }
}
