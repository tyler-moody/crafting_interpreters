use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version)]
struct Args {
    /// Lox script to execute
    script: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    match args.script {
        None => run_prompt(),
        Some(script) => run_file(script),
    }
}

fn run_file(script: PathBuf) {
    println!(
        "executing {}",
        script.into_os_string().into_string().unwrap()
    );
}

fn run_prompt() {
    println!("prompt");
}
