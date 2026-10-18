mod assembler;
mod lexer;
mod parser;

use crate::parser::Assembly;

use clap::Parser;
use std::fs;
use std::process;

use std::fs::File;
use std::io::{self, Write};
use std::collections::HashMap;
use serde::Serialize;

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

#[derive(Serialize)]
struct Config {
    label_map_hex: HashMap<String, String>,
}

fn to_hex_map(map: &HashMap<&str, i32>, base: u32) -> HashMap<String, String> {
    map.iter()
        .map(|(&k, &v)| (k.into(), format!("0x{:08x}", 4 * (v as u32) + base)))
        .collect()
}

fn main() {
    let start_addr: u32 = 0x8000_0000;
    let to_host: u32 = 0x7000_00f8;

    let cli = Cli::parse();

    let asm = fs::read_to_string(&cli.input)
        .unwrap_or_else(|e| exit(&format!("failed to open assembly ({}): {}", cli.input, e)));

    let tokens =
        lexer::tokenize(&asm).unwrap_or_else(|e| exit(&format!("failed to tokenize: {}", e)));

    let mut assembly = Vec::new();
    let (a, label_map) = parser::parse(&tokens).unwrap_or_else(|e| exit(&format!("failed to parse: {}", e)));
    assembly.extend(a);

    if cli.spike_sim {
        // spike sim終了用
        assembly.extend(vec![
            Assembly::LUI(1, (to_host & 0xFFFF_F000) >> 12),
            Assembly::ADDI(1, 1, (to_host & 0x0000_0FFF) as i32),
            Assembly::ADDI(2, 0, 0x0000_0001),
            Assembly::SW(2, 0, 1),
            Assembly::EBREAK,
        ])
    } else {
        assembly.push(Assembly::EBREAK);
    }

    let bin = assembler::assemble(assembly)
        .unwrap_or_else(|e| exit(&format!("failed to assemble: {}", e)));
    let _ = write_to_bin(&cli.output, &bin);

    let hex_map = to_hex_map(&label_map, start_addr);
    let cfg = Config { label_map_hex: hex_map };
    
    let yaml_str = serde_yml::to_string(&cfg)
        .unwrap_or_else(|e| exit(&format!("failed to serialize yaml: {}", e)));
    
    fs::write(format!("{}.label.yaml", cli.output), yaml_str)
        .unwrap_or_else(|e| exit(&format!("failed to write label_map.yaml: {}", e)));
}