mod assembler;
mod lexer;
mod parser;

use crate::parser::Assembly;

use clap::Parser;
use std::fs;
use std::process;

use std::fs::File;
use std::io::{self, Write};

// little endian
pub fn write_to_bin(path: &str, bin: &[u32]) -> io::Result<()> {
    let mut file = File::create(path)?;
    for &inst in bin {
        file.write_all(&inst.to_le_bytes())?;
    }
    Ok(())
}

#[derive(Parser)]
#[command(author, version, about = "", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "")]
    input: String,
    #[arg(short, long)]
    output: String,
    #[arg(short, long)]
    spike_sim: bool,
}

fn exit(msg: &str) -> ! {
    eprintln!("Error: {}", msg);
    process::exit(1);
}

fn main() {
    let cli = Cli::parse();

    let asm = fs::read_to_string(&cli.input)
        .unwrap_or_else(|e| exit(&format!("failed to open assembly ({}): {}", cli.input, e)));

    let tokens =
        lexer::tokenize(&asm).unwrap_or_else(|e| exit(&format!("failed to tokenize: {}", e)));

    let mut assembly = Vec::new();
    assembly.extend(
        parser::parse(&tokens).unwrap_or_else(|e| exit(&format!("failed to parse: {}", e))),
    );
    if cli.spike_sim {
        // spike sim終了用
        let to_host: u32 = 0x2000_0000;
        assembly.extend(vec![
            Assembly::LUI(1, (to_host & 0xFFFF_F000) >> 12),
            Assembly::ADDI(1, 1, (to_host & 0x0000_0FFF).try_into().unwrap()),
            Assembly::ADDI(2, 0, 0x0000_0001),
            Assembly::SW(2, 0, 1),
            Assembly::EBREAK,
        ])
    }

    // println!("{:?}", assembly);

    let bin = assembler::assemble(assembly)
        .unwrap_or_else(|e| exit(&format!("failed to assemble: {}", e)));

    let _ = write_to_bin(&cli.output, &bin);
}
