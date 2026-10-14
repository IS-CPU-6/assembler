use std::collections::HashMap;

use crate::lexer::Token;
use crate::lexer::Token::*;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum Assembly {
    LUI(u32, u32),
    AUIPC(u32, u32),
    JAL(u32, i32),
    JALR(u32, i32, u32),
    //
    BEQ(u32, u32, i32),
    BNE(u32, u32, i32),
    BLT(u32, u32, i32),
    BGE(u32, u32, i32),
    BLTU(u32, u32, i32),
    BGEU(u32, u32, i32),
    //
    LB(u32, i32, u32),
    LH(u32, i32, u32),
    LW(u32, i32, u32),
    LBU(u32, i32, u32),
    LHU(u32, i32, u32),
    SB(u32, i32, u32),
    SH(u32, i32, u32),
    SW(u32, i32, u32),
    //
    ADDI(u32, u32, i32),
    SLTI(u32, u32, i32),
    SLTIU(u32, u32, i32),
    XORI(u32, u32, i32),
    ORI(u32, u32, i32),
    ANDI(u32, u32, i32),
    SLLI(u32, u32, u32),
    SRLI(u32, u32, u32),
    SRAI(u32, u32, u32),
    ADD(u32, u32, u32),
    SUB(u32, u32, u32),
    SLL(u32, u32, u32),
    SLT(u32, u32, u32),
    SLTU(u32, u32, u32),
    XOR(u32, u32, u32),
    SRL(u32, u32, u32),
    SRA(u32, u32, u32),
    OR(u32, u32, u32),
    AND(u32, u32, u32),
    //
    EBREAK,
    //
    MUL(u32, u32, u32),
    MULH(u32, u32, u32),
    MULHSU(u32, u32, u32),
    MULHU(u32, u32, u32),
    DIV(u32, u32, u32),
    DIVU(u32, u32, u32),
    REM(u32, u32, u32),
    REMU(u32, u32, u32),
    //
    FLW(u32, i32, u32),
    FSW(u32, i32, u32),
    FADD_S(u32, u32, u32),
    FSUB_S(u32, u32, u32),
    FMUL_S(u32, u32, u32),
    FDIV_S(u32, u32, u32),
    FSQRT_S(u32, u32),
    FMIN_S(u32, u32, u32),
    FMAX_S(u32, u32, u32),
    FMADD_S(u32, u32, u32, u32),
    FMSUB_S(u32, u32, u32, u32),
    FNMSUB_S(u32, u32, u32, u32),
    FNMADD_S(u32, u32, u32, u32),
    //
    FSGNJ_S(u32, u32, u32),
    FSGNJN_S(u32, u32, u32),
    FSGNJX_S(u32, u32, u32),
    //
    FLE_S(u32, u32, u32),
    FLT_S(u32, u32, u32),
    FEQ_S(u32, u32, u32),
    //
    FCVT_W_S(u32, u32),
    FCVT_WU_S(u32, u32),
    FCVT_S_W(u32, u32),
    FCVT_S_WU(u32, u32),
    FMV_X_W(u32, u32),
    FMV_W_X(u32, u32),
    //
    FCLASS_S(u32, u32),
    //
    FNORM_S(u32, u32, u32, u32),
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
            [] => {}
            _ => {
                line += 1;
            }
        }
        one_line = vec![];
    }

    // println!("{:?}", label_map);

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

            [Id("lui"), Reg(rd), Comma, Int(imm)] => vec![LUI(*rd, *imm as u32)],
            [Id("auipc"), Reg(rd), Comma, Int(imm)] => vec![AUIPC(*rd, *imm as u32)],
            [Id("jal"), Reg(rd), Comma, dest] => {
                vec![JAL(
                    *rd,
                    match dest {
                        Int(offset) => *offset >> 2,
                        Id(label) => {
                            label_map.get(label).copied().unwrap_or(0) - assembly.len() as i32
                        }
                        _ => {
                            return Err(format!("{:?}: invalid branch dest", one_line));
                        }
                    },
                )]
            }
            [
                Id("jalr"),
                Reg(rd),
                Comma,
                Int(offset),
                Open,
                Reg(rs1),
                Close,
            ] => {
                vec![JALR(*rd, *offset >> 2, *rs1)]
            }

            [Id("beq"), Reg(rs1), Comma, Reg(rs2), Comma, dest] => vec![BEQ(
                *rs1,
                *rs2,
                match dest {
                    Id(label) => label_map.get(label).copied().unwrap_or(0) - assembly.len() as i32,
                    Int(offset) => offset >> 2,
                    _ => {
                        return Err(format!("{:?}: invalid branch dest", one_line));
                    }
                },
            )],
            [Id("bne"), Reg(rs1), Comma, Reg(rs2), Comma, dest] => vec![BNE(
                *rs1,
                *rs2,
                match dest {
                    Id(label) => label_map.get(label).copied().unwrap_or(0) - assembly.len() as i32,
                    Int(offset) => offset >> 2,
                    _ => {
                        return Err(format!("{:?}: invalid branch dest", one_line));
                    }
                },
            )],
            [Id("blt"), Reg(rs1), Comma, Reg(rs2), Comma, dest] => vec![BLT(
                *rs1,
                *rs2,
                match dest {
                    Id(label) => label_map.get(label).copied().unwrap_or(0) - assembly.len() as i32,
                    Int(offset) => offset >> 2,
                    _ => {
                        return Err(format!("{:?}: invalid branch dest", one_line));
                    }
                },
            )],
            [Id("bge"), Reg(rs1), Comma, Reg(rs2), Comma, dest] => vec![BGE(
                *rs1,
                *rs2,
                match dest {
                    Id(label) => label_map.get(label).copied().unwrap_or(0) - assembly.len() as i32,
                    Int(offset) => offset >> 2,
                    _ => {
                        return Err(format!("{:?}: invalid branch dest", one_line));
                    }
                },
            )],
            [Id("bltu"), Reg(rs1), Comma, Reg(rs2), Comma, dest] => vec![BLTU(
                *rs1,
                *rs2,
                match dest {
                    Id(label) => label_map.get(label).copied().unwrap_or(0) - assembly.len() as i32,
                    Int(offset) => offset >> 2,
                    _ => {
                        return Err(format!("{:?}: invalid branch dest", one_line));
                    }
                },
            )],
            [Id("bgeu"), Reg(rs1), Comma, Reg(rs2), Comma, dest] => vec![BGEU(
                *rs1,
                *rs2,
                match dest {
                    Id(label) => label_map.get(label).copied().unwrap_or(0) - assembly.len() as i32,
                    Int(offset) => offset >> 2,
                    _ => {
                        return Err(format!("{:?}: invalid branch dest", one_line));
                    }
                },
            )],

            [Id("lb"), Reg(rd), Comma, Int(offset), Open, Reg(rs1), Close] => {
                vec![LB(*rd, ((*offset) as i32).into(), *rs1)]
            }
            [Id("lh"), Reg(rd), Comma, Int(offset), Open, Reg(rs1), Close] => {
                vec![LH(*rd, ((*offset) as i32).into(), *rs1)]
            }
            [Id("lw"), Reg(rd), Comma, Int(offset), Open, Reg(rs1), Close] => {
                vec![LW(*rd, ((*offset) as i32).into(), *rs1)]
            }
            [
                Id("lbu"),
                Reg(rd),
                Comma,
                Int(offset),
                Open,
                Reg(rs1),
                Close,
            ] => {
                vec![LBU(*rd, ((*offset) as i32).into(), *rs1)]
            }
            [
                Id("lhu"),
                Reg(rd),
                Comma,
                Int(offset),
                Open,
                Reg(rs1),
                Close,
            ] => {
                vec![LHU(*rd, ((*offset) as i32).into(), *rs1)]
            }
            [
                Id("sb"),
                Reg(rs2),
                Comma,
                Int(offset),
                Open,
                Reg(rs1),
                Close,
            ] => {
                vec![SB(*rs2, ((*offset) as i32).into(), *rs1)]
            }
            [
                Id("sh"),
                Reg(rs2),
                Comma,
                Int(offset),
                Open,
                Reg(rs1),
                Close,
            ] => {
                vec![SH(*rs2, ((*offset) as i32).into(), *rs1)]
            }
            [
                Id("sw"),
                Reg(rs2),
                Comma,
                Int(offset),
                Open,
                Reg(rs1),
                Close,
            ] => {
                vec![SW(*rs2, ((*offset) as i32).into(), *rs1)]
            }

            [Id("addi"), Reg(rd), Comma, Reg(r1), Comma, Int(imm)] => {
                vec![ADDI(*rd, *r1, (*imm) as i32)]
            }
            [Id("slti"), Reg(rd), Comma, Reg(r1), Comma, Int(imm)] => {
                vec![SLTI(*rd, *r1, (*imm) as i32)]
            }
            [Id("sltiu"), Reg(rd), Comma, Reg(r1), Comma, Int(imm)] => {
                vec![SLTIU(*rd, *r1, (*imm) as i32)]
            }
            [Id("xori"), Reg(rd), Comma, Reg(r1), Comma, Int(imm)] => {
                vec![XORI(*rd, *r1, (*imm) as i32)]
            }
            [Id("ori"), Reg(rd), Comma, Reg(r1), Comma, Int(imm)] => {
                vec![ORI(*rd, *r1, (*imm) as i32)]
            }
            [Id("andi"), Reg(rd), Comma, Reg(r1), Comma, Int(imm)] => {
                vec![ANDI(*rd, *r1, (*imm) as i32)]
            }
            [Id("slli"), Reg(rd), Comma, Reg(r1), Comma, Int(imm)] => {
                vec![SLLI(*rd, *r1, (*imm) as u32)]
            }
            [Id("srli"), Reg(rd), Comma, Reg(r1), Comma, Int(imm)] => {
                vec![SRLI(*rd, *r1, (*imm) as u32)]
            }
            [Id("srai"), Reg(rd), Comma, Reg(r1), Comma, Int(imm)] => {
                vec![SRAI(*rd, *r1, (*imm) as u32)]
            }

            [Id("add"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![ADD(*rd, *r1, *r2)],
            [Id("sub"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![SUB(*rd, *r1, *r2)],
            [Id("sll"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![SLL(*rd, *r1, *r2)],
            [Id("slt"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![SLT(*rd, *r1, *r2)],
            [Id("sltu"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![SLTU(*rd, *r1, *r2)],
            [Id("xor"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![XOR(*rd, *r1, *r2)],
            [Id("srl"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![SRL(*rd, *r1, *r2)],
            [Id("sra"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![SRA(*rd, *r1, *r2)],
            [Id("or"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![OR(*rd, *r1, *r2)],
            [Id("and"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![AND(*rd, *r1, *r2)],

            [Id("ebreak")] => vec![EBREAK],

            //
            [Id("mul"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![MUL(*rd, *r1, *r2)],
            [Id("mulh"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![MULH(*rd, *r1, *r2)],
            [Id("mulhsu"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![MULHSU(*rd, *r1, *r2)],
            [Id("mulhu"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![MULHU(*rd, *r1, *r2)],
            [Id("div"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![DIV(*rd, *r1, *r2)],
            [Id("divu"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![DIVU(*rd, *r1, *r2)],
            [Id("rem"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![REM(*rd, *r1, *r2)],
            [Id("remu"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![REMU(*rd, *r1, *r2)],

            //
            [
                Id("flw"),
                Reg(rd),
                Comma,
                Int(offset),
                Open,
                Reg(rs1),
                Close,
            ] => {
                vec![FLW(*rd, *offset >> 2, *rs1)]
            }
            [
                Id("fsw"),
                Reg(rs2),
                Comma,
                Int(offset),
                Open,
                Reg(rs1),
                Close,
            ] => {
                vec![FSW(*rs2, *offset >> 2, *rs1)]
            }
            [Id("fadd.s"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![FADD_S(*rd, *r1, *r2)],
            [Id("fsub.s"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![FSUB_S(*rd, *r1, *r2)],
            [Id("fmul.s"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![FMUL_S(*rd, *r1, *r2)],
            [Id("fdiv.s"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![FDIV_S(*rd, *r1, *r2)],
            [Id("fsqrt.s"), Reg(rd), Comma, Reg(r1)] => vec![FSQRT_S(*rd, *r1)],
            [Id("fmin.s"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![FMIN_S(*rd, *r1, *r2)],
            [Id("fmax.s"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![FMAX_S(*rd, *r1, *r2)],
            [
                Id("fmadd.s"),
                Reg(rd),
                Comma,
                Reg(r1),
                Comma,
                Reg(r2),
                Comma,
                Reg(r3),
            ] => vec![FMADD_S(*rd, *r1, *r2, *r3)],
            [
                Id("fmsub.s"),
                Reg(rd),
                Comma,
                Reg(r1),
                Comma,
                Reg(r2),
                Comma,
                Reg(r3),
            ] => vec![FMSUB_S(*rd, *r1, *r2, *r3)],
            [
                Id("fnmsub.s"),
                Reg(rd),
                Comma,
                Reg(r1),
                Comma,
                Reg(r2),
                Comma,
                Reg(r3),
            ] => vec![FNMSUB_S(*rd, *r1, *r2, *r3)],
            [
                Id("fnmadd.s"),
                Reg(rd),
                Comma,
                Reg(r1),
                Comma,
                Reg(r2),
                Comma,
                Reg(r3),
            ] => vec![FNMADD_S(*rd, *r1, *r2, *r3)],
            [Id("fsgnj.s"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => {
                vec![FSGNJ_S(*rd, *r1, *r2)]
            }
            [Id("fsgnjn.s"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => {
                vec![FSGNJN_S(*rd, *r1, *r2)]
            }
            [Id("fsgnjx.s"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => {
                vec![FSGNJX_S(*rd, *r1, *r2)]
            }
            [Id("fle.s"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![FLE_S(*rd, *r1, *r2)],
            [Id("flt.s"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![FLT_S(*rd, *r1, *r2)],
            [Id("feq.s"), Reg(rd), Comma, Reg(r1), Comma, Reg(r2)] => vec![FEQ_S(*rd, *r1, *r2)],
            [Id("fcvt.w.s"), Reg(rd), Comma, Reg(r1)] => vec![FCVT_W_S(*rd, *r1)],
            [Id("fcvt.wu.s"), Reg(rd), Comma, Reg(r1)] => vec![FCVT_WU_S(*rd, *r1)],
            [Id("fcvt.s.w"), Reg(rd), Comma, Reg(r1)] => vec![FCVT_S_W(*rd, *r1)],
            [Id("fcvt.s.wu"), Reg(rd), Comma, Reg(r1)] => vec![FCVT_S_WU(*rd, *r1)],
            [Id("fmv.x.w"), Reg(rd), Comma, Reg(r1)] => vec![FMV_X_W(*rd, *r1)],
            [Id("fmv.w.x"), Reg(rd), Comma, Reg(r1)] => vec![FMV_W_X(*rd, *r1)],
            [Id("fclass.s"), Reg(rd), Comma, Reg(r1)] => vec![FCLASS_S(*rd, *r1)],
            [
                Id("fnorm.s"),
                Reg(rd),
                Comma,
                Reg(r1),
                Comma,
                Reg(r2),
                Comma,
                Reg(r3),
            ] => vec![FNORM_S(*rd, *r1, *r2, *r3)],
            [] => vec![],
            _ => {
                return Err(format!(
                    "invalid expression in line {}: {:?}",
                    line, one_line
                ));
            }
        };
        assembly.extend(asm);
        one_line.clear();
    }

    Ok(assembly)
}
