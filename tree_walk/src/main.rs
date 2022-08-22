use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

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
    println!("prompt");
    Ok(())
}

fn run(source: String) {
    println!("{}", source);
}
