mod assembler;
mod lexer;
mod parser;

use clap::Parser;
use std::fs;
use std::process;

#[derive(Parser)]
#[command(author, version, about = "", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "")]
    input: String,
    // #[arg(short, long)]
    // output: Option<String>,
}

fn exit(msg: &str) -> ! {
    eprintln!("Error: {}", msg);
    process::exit(1);
}

fn main() {
    let cli = Cli::parse();

    let asm = fs::read_to_string(&cli.input)
        .unwrap_or_else(|e| exit(&format!("failed to open assembly ({}): {}", cli.input, e)));

    println!("{}", asm);

    let tokens =
        lexer::tokenize(&asm).unwrap_or_else(|e| exit(&format!("failed to tokenize: {}", e)));

    let assembly =
        parser::parse(&tokens).unwrap_or_else(|e| exit(&format!("failed to parse: {}", e)));

    let bin = assembler::assemble(assembly)
        .unwrap_or_else(|e| exit(&format!("failed to assemble: {}", e)));

    println!("{:?}", bin);
}
