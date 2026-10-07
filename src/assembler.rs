use crate::parser::Assembly;
use crate::parser::Assembly::*;

#[derive(Debug, Clone, Copy)]
#[rustfmt::skip]
pub enum Format {
    R { funct7: u32, rs2: u32, rs1: u32, funct3: u32, rd: u32, opcode: u32,},
    I { imm: u32, rs1: u32, funct3: u32, rd: u32, opcode: u32,},
    S { imm: u32, rs2: u32, rs1: u32, funct3: u32, opcode: u32,},
    B { imm: u32, rs2: u32, rs1: u32, funct3: u32, opcode: u32,},
    U { imm: u32, rd: u32, opcode: u32,},
    J { imm: u32, rd: u32, opcode: u32,},
    R4 { rs3: u32, funct2: u32, rs2: u32, rs1: u32, funct3: u32, rd: u32, opcode: u32,},
}

#[rustfmt::skip]
impl Format {
    pub fn as_u32(&self) -> Result<u32, String> {
        let mask = |bits: u32| (1u32 << bits) - 1;

        match *self {
            Format::R { funct7, rs2, rs1, funct3, rd, opcode } => {
                let funct7 = funct7 & mask(7);
                let rs2    = rs2    & mask(5);
                let rs1    = rs1    & mask(5);
                let funct3 = funct3 & mask(3);
                let rd     = rd     & mask(5);
                let opcode = opcode & mask(7);

                Ok((funct7 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode)
            }
            Format::I { imm, rs1, funct3, rd, opcode } => {
                let imm    = imm    & mask(12);
                let rs1    = rs1    & mask(5);
                let funct3 = funct3 & mask(3);
                let rd     = rd     & mask(5);
                let opcode = opcode & mask(7);

                Ok((imm << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode)
            }
            Format::S { imm, rs2, rs1, funct3, opcode } => {
                let imm    = imm    & mask(12);
                let rs2    = rs2    & mask(5);
                let rs1    = rs1    & mask(5);
                let funct3 = funct3 & mask(3);
                let opcode = opcode & mask(7);

                let imm11_5 = (imm >> 5) & mask(7);
                let imm4_0  = imm & mask(5);

                Ok((imm11_5 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (imm4_0 << 7) | opcode)
            }
            Format::B { imm, rs2, rs1, funct3, opcode } => {
                let imm    = imm    & mask(13);
                let rs2    = rs2    & mask(5);
                let rs1    = rs1    & mask(5);
                let funct3 = funct3 & mask(3);
                let opcode = opcode & mask(7);

                let imm12   = (imm >> 12) & mask(1);
                let imm10_5 = (imm >> 5)  & mask(6);
                let imm4_1  = (imm >> 1)  & mask(4);
                let imm11   = (imm >> 11) & mask(1);

                Ok((imm12 << 31) | (imm10_5 << 25) | (rs2 << 20) | (rs1 << 15) 
                    | (funct3 << 12) | (imm4_1 << 8) | (imm11 << 7) | opcode)
            }
            Format::U { imm, rd, opcode } => {
                let imm    = imm    & mask(20);
                let rd     = rd     & mask(5);
                let opcode = opcode & mask(7);

                Ok((imm << 12) | (rd << 7) | opcode)
            }
            Format::J { imm, rd, opcode } => {
                let imm    = imm    & mask(21);
                let rd     = rd     & mask(5);
                let opcode = opcode & mask(7);

                let imm20   = (imm >> 20) & mask(1);
                let imm10_1 = (imm >> 1)  & mask(10);
                let imm11   = (imm >> 11) & mask(1);
                let imm19_12= (imm >> 12) & mask(8);

                Ok((imm20 << 31) | (imm10_1 << 21) | (imm11 << 20) | (imm19_12 << 12) | (rd << 7) | opcode)
            }
            Format::R4 { rs3, funct2, rs2, rs1, funct3, rd, opcode } => {
                let rs3    = rs3    & mask(5);
                let funct2 = funct2 & mask(2);
                let rs2    = rs2    & mask(5);
                let rs1    = rs1    & mask(5);
                let funct3 = funct3 & mask(3);
                let rd     = rd     & mask(5);
                let opcode = opcode & mask(7);

                Ok((rs3 << 27) | (funct2 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode)
            }
        }
    }
}

#[rustfmt::skip]
pub fn assemble(assembly: Vec<Assembly>) -> Result<Vec<u32>, String> {
    let mut res = Vec::new();
    for asm in assembly {
        let formatted = match asm {
            LUI(rd, imm) => Format::U { imm, rd, opcode: 0b0110111 },
            AUIPC(rd, imm) => Format::U { imm, rd, opcode: 0b0010111 },
            JAL(rd, ofs) => Format::J { imm: (ofs << 2) as u32, rd, opcode: 0b1101111 },
            JALR(rd, ofs, rs1) => Format::I { imm: (ofs << 2) as u32, rs1, funct3: 0b000, rd, opcode: 0b1100111 },

            BEQ(rs1, rs2, ofs) => Format::B { imm: (ofs << 2) as u32, rs2, rs1, funct3: 0b000, opcode: 0b1100011 },
            BNE(rs1, rs2, ofs) => Format::B { imm: (ofs << 2) as u32, rs2, rs1, funct3: 0b001, opcode: 0b1100011 },
            BLT(rs1, rs2, ofs) => Format::B { imm: (ofs << 2) as u32, rs2, rs1, funct3: 0b100, opcode: 0b1100011 },
            BGE(rs1, rs2, ofs) => Format::B { imm: (ofs << 2) as u32, rs2, rs1, funct3: 0b101, opcode: 0b1100011 },
            BLTU(rs1, rs2, ofs) => Format::B { imm: (ofs << 2) as u32, rs2, rs1, funct3: 0b110, opcode: 0b1100011 },
            BGEU(rs1, rs2, ofs) => Format::B { imm: (ofs << 2) as u32, rs2, rs1, funct3: 0b111, opcode: 0b1100011 },

            LB(rd, ofs, rs1) => Format::I { imm: ofs as u32, rs1, funct3: 0b000, rd, opcode: 0b0000011 },
            LH(rd, ofs, rs1) => Format::I { imm: ofs as u32, rs1, funct3: 0b001, rd, opcode: 0b0000011 },
            LW(rd, ofs, rs1) => Format::I { imm: ofs as u32, rs1, funct3: 0b010, rd, opcode: 0b0000011 },
            LBU(rd, ofs, rs1) => Format::I { imm: ofs as u32, rs1, funct3: 0b100, rd, opcode: 0b0000011 },
            LHU(rd, ofs, rs1) => Format::I { imm: ofs as u32, rs1, funct3: 0b101, rd, opcode: 0b0000011 },
            SB(rs2, ofs, rs1) => Format::S { imm: ofs as u32, rs2, rs1, funct3: 0b000, opcode: 0b0100011 },
            SH(rs2, ofs, rs1) => Format::S { imm: ofs as u32, rs2, rs1, funct3: 0b001, opcode: 0b0100011 },
            SW(rs2, ofs, rs1) => Format::S { imm: ofs as u32, rs2, rs1, funct3: 0b010, opcode: 0b0100011 },

            ADDI(rd, rs1, imm) => Format::I { imm: imm as u32, rs1, funct3: 0b000, rd, opcode: 0b0010011 },
            SLTI(rd, rs1, imm) => Format::I { imm: imm as u32, rs1, funct3: 0b010, rd, opcode: 0b0010011 },
            SLTIU(rd, rs1, imm) => Format::I { imm: imm as u32, rs1, funct3: 0b011, rd, opcode: 0b0010011 },
            XORI(rd, rs1, imm) => Format::I { imm: imm as u32, rs1, funct3: 0b100, rd, opcode: 0b0010011 },
            ORI(rd, rs1, imm) => Format::I { imm: imm as u32, rs1, funct3: 0b110, rd, opcode: 0b0010011 },
            ANDI(rd, rs1, imm) => Format::I { imm: imm as u32, rs1, funct3: 0b111, rd, opcode: 0b0010011 },
            SLLI(rd, rs1, shamt) => Format::I { imm: shamt, rs1, funct3: 0b001, rd, opcode: 0b0010011 },
            SRLI(rd, rs1, shamt) => Format::I { imm: shamt, rs1, funct3: 0b101, rd, opcode: 0b0010011 },
            SRAI(rd, rs1, shamt) => Format::I { imm: (0b0100000 << 5) | shamt, rs1, funct3: 0b101, rd, opcode: 0b0010011 },
            
            ADD(rd, rs1, rs2) => Format::R { funct7: 0b0000000, rs2, rs1, funct3: 0b000, rd, opcode: 0b0110011 },
            SUB(rd, rs1, rs2) => Format::R { funct7: 0b0100000, rs2, rs1, funct3: 0b000, rd, opcode: 0b0110011 },
            SLL(rd, rs1, rs2) => Format::R { funct7: 0b0000000, rs2, rs1, funct3: 0b001, rd, opcode: 0b0110011 },
            SLT(rd, rs1, rs2) => Format::R { funct7: 0b0000000, rs2, rs1, funct3: 0b010, rd, opcode: 0b0110011 },
            SLTU(rd, rs1, rs2) => Format::R { funct7: 0b0000000, rs2, rs1, funct3: 0b011, rd, opcode: 0b0110011 },
            XOR(rd, rs1, rs2) => Format::R { funct7: 0b0000000, rs2, rs1, funct3: 0b100, rd, opcode: 0b0110011 },
            SRL(rd, rs1, rs2) => Format::R { funct7: 0b0000000, rs2, rs1, funct3: 0b101, rd, opcode: 0b0110011 },
            SRA(rd, rs1, rs2) => Format::R { funct7: 0b0100000, rs2, rs1, funct3: 0b101, rd, opcode: 0b0110011 },
            OR(rd, rs1, rs2) => Format::R { funct7: 0b0000000, rs2, rs1, funct3: 0b110, rd, opcode: 0b0110011 },
            AND(rd, rs1, rs2) => Format::R { funct7: 0b0000000, rs2, rs1, funct3: 0b111, rd, opcode: 0b0110011 },

            EBREAK => Format::I { imm: 0b0000_0000_0001, rs1: 0b00000, funct3: 0b000, rd: 0b00000, opcode: 0b1110011 },

            MUL(rd, rs1, rs2) => Format::R { funct7: 0b0000001, rs2, rs1, funct3: 0b000, rd, opcode: 0b0110011 },
            MULH(rd, rs1, rs2) => Format::R { funct7: 0b0000001, rs2, rs1, funct3: 0b001, rd, opcode: 0b0110011 },
            MULHSU(rd, rs1, rs2) => Format::R { funct7: 0b0000001, rs2, rs1, funct3: 0b010, rd, opcode: 0b0110011 },
            MULHU(rd, rs1, rs2) => Format::R { funct7: 0b0000001, rs2, rs1, funct3: 0b011, rd, opcode: 0b0110011 },
            DIV(rd, rs1, rs2) => Format::R { funct7: 0b0000001, rs2, rs1, funct3: 0b100, rd, opcode: 0b0110011 },
            DIVU(rd, rs1, rs2) => Format::R { funct7: 0b0000001, rs2, rs1, funct3: 0b101, rd, opcode: 0b0110011 },
            REM(rd, rs1, rs2) => Format::R { funct7: 0b0000001, rs2, rs1, funct3: 0b110, rd, opcode: 0b0110011 },
            REMU(rd, rs1, rs2) => Format::R { funct7: 0b0000001, rs2, rs1, funct3: 0b111, rd, opcode: 0b0110011 },

            FLW(rd, ofs, rs1) => Format::I { imm: ofs as u32, rs1, funct3: 0b010, rd, opcode: 0b0000111 },
            FSW(rs2, ofs, rs1) => Format::S { imm: ofs as u32, rs2, rs1, funct3: 0b010, opcode: 0b0100111 },
            FADD_S(rd, rs1, rs2) => Format::R { funct7: 0b0000000, rs2, rs1, funct3: 0b000, rd, opcode: 0b1010011 },
            FSUB_S(rd, rs1, rs2) => Format::R { funct7: 0b0000100, rs2, rs1, funct3: 0b000, rd, opcode: 0b1010011 },
            FMUL_S(rd, rs1, rs2) => Format::R { funct7: 0b0001000, rs2, rs1, funct3: 0b000, rd, opcode: 0b1010011 },
            FDIV_S(rd, rs1, rs2) => Format::R { funct7: 0b0001100, rs2, rs1, funct3: 0b000, rd, opcode: 0b1010011 },
            FSQRT_S(rd, rs1) => Format::R { funct7: 0b0101100, rs2: 0b00000, rs1, funct3: 0b000, rd, opcode: 0b1010011 },
            FMIN_S(rd, rs1, rs2) => Format::R { funct7: 0b0010100, rs2, rs1, funct3: 0b000, rd, opcode: 0b1010011 },
            FMAX_S(rd, rs1, rs2) => Format::R { funct7: 0b0010100, rs2, rs1, funct3: 0b001, rd, opcode: 0b1010011 },
            
            FMADD_S(rd, rs1, rs2, rs3) => Format::R4 { rs3, funct2: 0b00, rs2, rs1, funct3: 0b000, rd, opcode: 0b1000011 },
            FMSUB_S(rd, rs1, rs2, rs3) => Format::R4 { rs3, funct2: 0b00, rs2, rs1, funct3: 0b000, rd, opcode: 0b1000111 },
            FNMSUB_S(rd, rs1, rs2, rs3) => Format::R4 { rs3, funct2: 0b00, rs2, rs1, funct3: 0b000, rd, opcode: 0b1001011 },
            FNMADD_S(rd, rs1, rs2, rs3) => Format::R4 { rs3, funct2: 0b00, rs2, rs1, funct3: 0b000, rd, opcode: 0b1001111 },

            FSGNJ_S(rd, rs1, rs2) => Format::R { funct7: 0b0010000, rs2, rs1, funct3: 0b000, rd, opcode: 0b1010011 },
            FSGNJN_S(rd, rs1, rs2) => Format::R { funct7: 0b0010000, rs2, rs1, funct3: 0b001, rd, opcode: 0b1010011 },
            FSGNJX_S(rd, rs1, rs2) => Format::R { funct7: 0b0010000, rs2, rs1, funct3: 0b010, rd, opcode: 0b1010011 },

            FLE_S(rd, rs1, rs2) => Format::R { funct7: 0b1010000, rs2, rs1, funct3: 0b000, rd, opcode: 0b1010011 },
            FLT_S(rd, rs1, rs2) => Format::R { funct7: 0b1010000, rs2, rs1, funct3: 0b001, rd, opcode: 0b1010011 },
            FEQ_S(rd, rs1, rs2) => Format::R { funct7: 0b1010000, rs2, rs1, funct3: 0b010, rd, opcode: 0b1010011 },

            FCVT_W_S(rd, rs1) => Format::R { funct7: 0b1100000, rs2: 0b00000, rs1, funct3: 0b000, rd, opcode: 0b1010011 },
            FCVT_WU_S(rd, rs1) => Format::R { funct7: 0b1100000, rs2: 0b00001, rs1, funct3: 0b000, rd, opcode: 0b1010011 },
            FCVT_S_W(rd, rs1) => Format::R { funct7: 0b1101000, rs2: 0b00000, rs1, funct3: 0b000, rd, opcode: 0b1010011 },
            FCVT_S_WU(rd, rs1) => Format::R { funct7: 0b1101000, rs2: 0b00001, rs1, funct3: 0b000, rd, opcode: 0b1010011 },
            FMV_X_W(rd, rs1) => Format::R { funct7: 0b1110000, rs2: 0b00000, rs1, funct3: 0b000, rd, opcode: 0b1010011 },
            FMV_W_X(rd, rs1) => Format::R { funct7: 0b1111000, rs2: 0b00000, rs1, funct3: 0b000, rd, opcode: 0b1010011 },

            FCLASS_S(rd, rs1) => Format::R { funct7: 0b1110000, rs2: 0b00000, rs1, funct3: 0b001, rd, opcode: 0b1010011 },

            FNORM_S(rd, rs1, rs2, rs3) => Format::R4 { rs3, funct2: 0b01, rs2, rs1, funct3: 0b000, rd, opcode: 0b1000011 },
        };
        match formatted.as_u32() {
            Ok(u) => {res.push(u);}
            Err(e) => {return Err(format!("{:?}: {}", asm, e));}
        }
    }
    Ok(res)
}
