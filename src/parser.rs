use std::collections::HashMap;

use crate::lexer::Token;
use crate::lexer::Token::*;

#[derive(Debug, Clone)]
pub enum Assembly {
    ADDI(u32, u32, u32),
    BEQ(u32, u32, i32),
    BNE(u32, u32, i32),
    BLT(u32, u32, i32),
    BGE(u32, u32, i32),
    BLTU(u32, u32, i32),
    BGEU(u32, u32, i32),
}
use Assembly::*;

pub fn parse<'s>(tokens: &Vec<Token<'s>>) -> Result<Vec<Assembly>, String> {
    let mut label_map: HashMap<&'s str, i32> = HashMap::new();
    let mut one_line = Vec::new();
    let mut line: i32 = 0;

    // label -> address
    for token in tokens {
        if *token != Return {
            one_line.push(token);
            continue;
        }
        match one_line[..] {
            [Id(label), Colon] => {
                label_map.insert(label, line);
            }
            _ => {
                line += 1;
            }
        }
    }

    println!("{:?}", label_map);

    // assemble
    let mut assembly = Vec::new();
    one_line = Vec::new();

    for token in tokens {
        if *token != Return {
            one_line.push(token);
            continue;
        }
        let asm: Vec<Assembly> = match one_line[..] {
            [Id(_), Colon] => vec![],
            [Id("addi"), Reg(rd), Comma, Reg(r1), Comma, Int(imm)] => vec![ADDI(*rd, *r1, *imm)],
            [Id("beq"), Reg(rs1), Comma, Reg(rs2), Comma, Id(label)] => vec![BEQ(
                *rs1,
                *rs2,
                label_map.get(label).copied().unwrap_or(0) - assembly.len() as i32,
            )],
            [Id("bne"), Reg(rs1), Comma, Reg(rs2), Comma, Id(label)] => vec![BNE(
                *rs1,
                *rs2,
                label_map.get(label).copied().unwrap_or(0) - assembly.len() as i32,
            )],
            [Id("blt"), Reg(rs1), Comma, Reg(rs2), Comma, Id(label)] => vec![BLT(
                *rs1,
                *rs2,
                label_map.get(label).copied().unwrap_or(0) - assembly.len() as i32,
            )],
            [Id("bge"), Reg(rs1), Comma, Reg(rs2), Comma, Id(label)] => vec![BGE(
                *rs1,
                *rs2,
                label_map.get(label).copied().unwrap_or(0) - assembly.len() as i32,
            )],
            [Id("bltu"), Reg(rs1), Comma, Reg(rs2), Comma, Id(label)] => vec![BLTU(
                *rs1,
                *rs2,
                label_map.get(label).copied().unwrap_or(0) - assembly.len() as i32,
            )],
            [Id("bgeu"), Reg(rs1), Comma, Reg(rs2), Comma, Id(label)] => vec![BGEU(
                *rs1,
                *rs2,
                label_map.get(label).copied().unwrap_or(0) - assembly.len() as i32,
            )],
            _ => {
                return Err(format!(
                    "invalid expression in line {}: {:?}",
                    line, one_line
                ));
            }
        };
        assembly.extend(asm);
        one_line = Vec::new();
    }

    Ok(assembly)
}
