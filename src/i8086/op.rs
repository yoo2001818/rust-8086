use std::iter::Iterator;

pub enum OpRegisterWord {
  AX,
  CX,
  DX,
  BX,
  SP,
  BP,
  SI,
  DI,
}

pub enum OpRegisterByte {
  AL,
  CL,
  DL,
  BL,
  AH,
  CH,
  DH,
  BH,
}

pub enum OpSegmentRegister {
  ES,
  CS,
  SS,
  DS,
}

pub enum OpAddressType {
  BX_SI,
  BX_DI,
  BP_SI,
  BP_DI,
  SI,
  DI,
  BP,
  BX,
}

pub enum OpModRm<T> {
  REGISTER(T),
  ADDRESS(OpAddressType),
  ADDRESS_DISP_BYTE(OpAddressType, u8),
  ADDRESS_DISP_WORD(OpAddressType, u16),
  DIRECT(u16),
}

pub type OpModRmWord = OpModRm<OpRegisterWord>;
pub type OpModRmByte = OpModRm<OpRegisterByte>;

pub enum OpModRmDual {
  WORD(OpModRmWord),
  BYTE(OpModRmByte),
}

pub enum OpDirectionType {
  REG_TO_RM,
  RM_TO_REG,
}

pub struct OpModRegRm<T>(T, OpModRm<T>);

pub type OpModRegRmWord = OpModRegRm<OpRegisterWord>;
pub type OpModRegRmByte = OpModRegRm<OpRegisterByte>;

pub enum OpModRegRmDual {
  WORD(OpModRegRmWord),
  BYTE(OpModRegRmByte),
}

pub enum OpBinarySrcDest<T, V> {
  REG_TO_RM(OpModRegRm<T>),
  RM_TO_REG(OpModRegRm<T>),
  IMM_RM(OpModRm<T>, V),
  IMM_REG(T, V),
  IMM_AX(V),
}

pub type OpBinarySrcDestWord = OpBinarySrcDest<OpRegisterWord, u16>;
pub type OpBinarySrcDestByte = OpBinarySrcDest<OpRegisterByte, u8>;

pub enum OpBinarySrcDestDual {
  WORD(OpBinarySrcDestWord),
  BYTE(OpBinarySrcDestByte),
}

pub enum OpRotateType {
  ONE,
  CL,
}

pub enum OpWordByte {
  BYTE,
  WORD,
}

pub enum Op {
  MOV(OpBinarySrcDestDual),
  MOV_WORD_SEG(OpDirectionType, OpModRmWord, OpSegmentRegister),
  MOV_AX_TO_MEM(u16),
  MOV_MEM_TO_AX(u16),
  MOV_AL_TO_MEM(u16),
  MOV_MEM_TO_AL(u16),
  PUSH_RM(OpModRmWord),
  PUSH_REG(OpRegisterWord),
  PUSH_SEG(OpSegmentRegister),
  POP_RM(OpModRmWord),
  POP_REG(OpRegisterWord),
  POP_SEG(OpSegmentRegister),
  XCHG_RM_REG(OpModRegRm),
  XCHG_REG_AX(OpRegisterWord),
  IN_FIXED(OpWordByte),
  IN_VARIABLE(OpWordByte, u8),
  OUT_FIXED(OpWordByte),
  OUT_VARIABLE(OpWordByte, u8),
  XLAT,
  LEA(OpModRegRmWord),
  LDS(OpModRegRmWord),
  LES(OpModRegRmWord),
  LAHF,
  SAHF,
  PUSHF,
  POPF,
  ADD(OpBinarySrcDestDual),
  ADC(OpBinarySrcDestDual),
  INC_RM(OpModRmDual),
  INC_REG(OpRegisterWord),
  AAA,
  DAA,
  SUB(OpBinarySrcDestDual),
  SBB(OpBinarySrcDestDual),
  DEC_RM(OpModRmDual),
  DEC_REG(OpRegisterWord),
  NEG(OpModRmDual),
  CMP(OpBinarySrcDestDual),
  AAS,
  DAS,
  MUL(OpModRmDual),
  IMUL(OpModRmDual),
  AAM,
  DIV(OpModRmDual),
  IDIV(OpModRmDual),
  AAD,
  CBW,
  CWD,
  NOT(OpModRmDual),
  SHL(OpRotateType, OpModRmDual),
  SHR(OpRotateType, OpModRmDual),
  SAR(OpRotateType, OpModRmDual),
  ROL(OpRotateType, OpModRmDual),
  ROR(OpRotateType, OpModRmDual),
  RCL(OpRotateType, OpModRmDual),
  RCR(OpRotateType, OpModRmDual),
  AND(OpBinarySrcDestDual),
  TEST(OpModRegRmDual),
  TEST_IMM_BYTE(OpModRmByte, u8),
  TEST_IMM_WORD(OpModRmWord, u16),
  OR(OpBinarySrcDestDual),
  XOR(OpBinarySrcDestDual),
  REP_SET,
  REP_UNSET,
  MOVS(OpWordByte),
  CMPS(OpWordByte),
  SCAS(OpWordByte),
  LODS(OpWordByte),
  STDS(OpWordByte),
  CALL_WITHIN_DIRECT(u16),
  CALL_WITHIN_INDIRECT(OpModRmWord),
  CALL_INTER_DIRECT(u16, u16),
  CALL_INTER_INDIRECT(OpModRmWord),
  JMP_WITHIN_DIRECT(u16),
  JMP_WITHIN_DIRECT_SHORT(u8),
  JMP_WITHIN_INDIRECT(OpModRmWord),
  JMP_INTER_DIRECT(u16, u16),
  JMP_INTER_INDIRECT(OpModRmWord),
  RET_WITHIN,
  RET_WITHIN_IMM(u16),
  RET_INTER,
  RET_INTER_IMM(u16),
  JE(u8),
  JL(u8),
  JLE(u8),
  JB(u8),
  JBE(u8),
  JP(u8),
  JO(u8),
  JS(u8),
  JNE(u8),
  JGE(u8),
  JG(u8),
  JAE(u8),
  JA(u8),
  JNP(u8),
  JNO(u8),
  JNS(u8),
  LOOP(u8),
  LOOPZ(u8),
  LOOPNZ(u8),
  JCXZ(u8),
  INT(u8),
  INTO,
  IRET,
  CLC,
  CMC,
  STC,
  CLD,
  STD,
  CLI,
  STI,
  HLT,
  WAIT,
  ESC(u8, OpModRmWord),
  LOCK,
  SEGMENT(OpSegmentRegister),
}

pub fn parseRegisterWord(byte: u8) -> OpRegisterWord {
  match byte {
    0 => OpRegisterWord::AX,
    1 => OpRegisterWord::CX,
    2 => OpRegisterWord::DX,
    3 => OpRegisterWord::BX,
    4 => OpRegisterWord::SP,
    5 => OpRegisterWord::BP,
    6 => OpRegisterWord::SI,
    7 => OpRegisterWord::DI,
    _ => OpRegisterWord::AX,
  }
}

pub fn parseRegisterByte(byte: u8) -> OpRegisterByte {
  match byte {
    0 => OpRegisterByte::AL,
    1 => OpRegisterByte::CL,
    2 => OpRegisterByte::DL,
    3 => OpRegisterByte::BL,
    4 => OpRegisterByte::AH,
    5 => OpRegisterByte::CH,
    6 => OpRegisterByte::DH,
    7 => OpRegisterByte::BH,
    _ => OpRegisterByte::AL,
  }
}

pub fn parseAddressType(byte: u8) -> OpAddressType {
  match byte {
    0 => OpAddressType::BX_SI,
    1 => OpAddressType::BX_DI,
    2 => OpAddressType::BP_SI,
    3 => OpAddressType::BP_DI,
    4 => OpAddressType::SI,
    5 => OpAddressType::DI,
    6 => OpAddressType::BP,
    7 => OpAddressType::BX,
    _ => OpAddressType::BX_SI,
  }
}

pub fn parseSegmentRegister(byte: u8) -> Option<OpSegmentRegister> {
  match byte {
    0 => Some(OpSegmentRegister::ES),
    1 => Some(OpSegmentRegister::CS),
    2 => Some(OpSegmentRegister::DS),
    3 => Some(OpSegmentRegister::SS),
    _ => None(),
  }
}

fn iterNextU16 (iter: &mut Iterator<u8>) -> Option<u16> {
  Some(iter.next()? as u16 +
    ((iter.next()? as u16) << 8))
}

pub fn parseModRmWord(byte: u8, iter: &mut Iterator<u8>) -> Option<OpModRmWord> {
  let mod_val = (byte >> 5) & 0x07;
  let rm_val = byte & 0x03;
  Some(match rm_val {
    0 => OpModRmWord::REGISTER(parseRegisterWord(mod_val)),
    1 => {
      let addr_type = parseAddressType(mod_val);
      if addr_type == OpAddressType::BP {
        OpModRmWord::DIRECT(iterNextU16(iter)?)
      } else {
        OpModRmWord::ADDRESS(addr_type)
      }
    }
    2 => OpModRmWord::ADDRESS_DISP_BYTE(parseAddressType(mod_val),
      iter.next()?),
    3 => OpModRmWord::ADDRESS_DISP_WORD(parseAddressType(mod_val),
      iterNextU16(iter)?),
  })
}

pub fn parseModRmByte(byte: u8, iter: &mut Iterator<u8>) -> Option<OpModRmByte> {
  let mod_val = (byte >> 5) & 0x07;
  let rm_val = byte & 0x03;
  Some(match rm_val {
    0 => OpModRmByte::REGISTER(parseRegisterByte(mod_val)),
    1 => {
      let addr_type = parseAddressType(mod_val);
      if addr_type == OpAddressType::BP {
        OpModRmByte::DIRECT(iterNextU16(iter)?)
      } else {
        OpModRmByte::ADDRESS(addr_type)
      }
    }
    2 => OpModRmByte::ADDRESS_DISP_BYTE(parseAddressType(mod_val),
      iter.next()?),
    3 => OpModRmByte::ADDRESS_DISP_WORD(parseAddressType(mod_val),
      iterNextU16(iter)?),
  })
}

pub fn parseModRegRmByte(
  byte: u8, iter: &mut Iterator<u8>,
) -> Option<OpModRegRmByte> {
  let reg = parseRegisterByte((byte >> 3) & 0x7);
  let rm = parseModRmByte(byte, iter)?;
  return Some(OpModRegRmByte(reg, rm));
}

pub fn parseModRegRmWord(
  byte: u8, iter: &mut Iterator<u8>,
) -> Option<OpModRegRmWord> {
  let reg = parseRegisterWord((byte >> 3) & 0x7);
  let rm = parseModRmWord(byte, iter)?;
  return Some(OpModRegRmWord(reg, rm));
}

pub fn parseBinarySrcDest(
  first: u8, iter: &mut Iterator<u8>,
) -> Option<OpBinarySrcDest> {
  Some(match first & 0x07 {
    0..=3 => {
      let second = iter.next()?;
      match first & 0x03 {
        0 => OpBinarySrcDest::BYTE(OpBinarySrcDestByte::REG_TO_RM(parseModRegRmByte(second, iter))),
        1 => OpBinarySrcDest::WORD(OpBinarySrcDestWord::REG_TO_RM(parseModRegRmWord(second, iter))),
        2 => OpBinarySrcDest::BYTE(OpBinarySrcDestByte::RM_TO_REG(parseModRegRmByte(second, iter))),
        3 => OpBinarySrcDest::WORD(OpBinarySrcDestWord::RM_TO_REG(parseModRegRmWord(second, iter))),
        _ => panic!("This should never happen"),
      }
    },
    4 => OpBinarySrcDest::BYTE(
      OpBinarySrcDestByte::IMM_AX(iter.next()?)),
    5 => OpBinarySrcDest::WORD(
      OpBinarySrcDestWord::IMM_AX(iterNextU16(iter)?)),
    _ => panic!("..."),
  })
}

pub fn parseOp(iter: &mut Iterator<u8>) -> Option<Op> {
  const first = iter.next()?;
  Some(match first & 0xf8 {
    0x00 => {
      // ADD, PUSH ES, POP ES
      match first & 0x07 {
        0..=5 => Op::ADD(parseBinarySrcDest(first, iter)?),
        6 => Op::PUSH_SEG(OpSegmentRegister::ES),
        7 => Op::POP_SEG(OpSegmentRegister::ES),
        _ => panic!(""),
      }
    }
    0x08 => {
      // OR, PUSH CS
      match first & 0x07 {
        0..=5 => Op::OR(parseBinarySrcDest(first, iter)?),
        6 => Op::PUSH_SEG(OpSegmentRegister::CS),
        7 => Op::POP_SEG(OpSegmentRegister::CS),
        _ => panic!(""),
      }
    },
    0x10 => {
      // ADC, PUSH SS, POP SS
      match first & 0x07 {
        0..=5 => Op::ADC(parseBinarySrcDest(first, iter)?),
        6 => Op::PUSH_SEG(OpSegmentRegister::SS),
        7 => Op::POP_SEG(OpSegmentRegister::SS),
        _ => panic!(""),
      }
    },
    0x18 => {
      // SBB, PUSH DS, POP DS
      match first & 0x07 {
        0..=5 => Op::SBB(parseBinarySrcDest(first, iter)?),
        6 => Op::PUSH_SEG(OpSegmentRegister::DS),
        7 => Op::POP_SEG(OpSegmentRegister::DS),
        _ => panic!(""),
      }
    },
    0x20 => {
      // AND, SELECT ES, DAA
      match first & 0x07 {
        0..=5 => Op::AND(parseBinarySrcDest(first, iter)?),
        6 => Op::SEGMENT(OpSegmentRegister::ES),
        7 => Op::DAA,
        _ => panic!(""),
      }
    },
    0x28 => {
      // SUB, SELECT CS, DAS
      match first & 0x07 {
        0..=5 => Op::SUB(parseBinarySrcDest(first, iter)?),
        6 => Op::SEGMENT(OpSegmentRegister::CS),
        7 => Op::DAS,
        _ => panic!(""),
      }
    },
    0x30 => {
      // XOR, SELECT SS, AAA
      match first & 0x07 {
        0..=5 => Op::XOR(parseBinarySrcDest(first, iter)?),
        6 => Op::SEGMENT(OpSegmentRegister::SS),
        7 => Op::AAA,
        _ => panic!(""),
      }
    },
    0x38 => {
      // CMP, SELECT DS, AAS
      match first & 0x07 {
        0..=5 => Op::CMP(parseBinarySrcDest(first, iter)?),
        6 => Op::SEGMENT(OpSegmentRegister::DS),
        7 => Op::AAS,
        _ => panic!(""),
      }
    },
    0x40 => {
      // INC
      Op::INC_REG(parseRegisterWord(first & 0x07))
    },
    0x48 => {
      // DEC
      Op::DEC_REG(parseRegisterWord(first & 0x07))
    },
    0x50 => {
      // PUSH
      Op::PUSH_REG(parseRegisterWord(first & 0x07))
    },
    0x58 => {
      // POP
      Op::POP_REG(parseRegisterWord(first & 0x07))
    },
    0x60 => {
      // not used
    },
    0x68 => {
      // not used
    },
    0x70 => {
      // JO, JNO, JB, JNB, JE, JNE, JBE, JNBE
      let second = iter.next()?;
      match first & 0x07 {
        0 => Op::JO(second),
        1 => Op::JNO(second),
        2 => Op::JB(second),
        3 => Op::JAE(second),
        4 => Op::JE(second),
        5 => Op::JNE(second),
        6 => Op::JBE(second),
        7 => Op::JA(second),
      }
    },
    0x78 => {
      // JS, JNS, JP, JNP, JL, JNL, JLE, JNLE
      let second = iter.next()?;
      match first & 0x07 {
        0 => Op::JS(second),
        1 => Op::JNS(second),
        2 => Op::JP(second),
        3 => Op::JNP(second),
        4 => Op::JL(second),
        5 => Op::JGE(second),
        6 => Op::JLE(second),
        7 => Op::JG(second),
      }
    },
    0x80 => {
      // ADD, OR, ADC, SBB, AND, SUB, XOR, CMP
      // 80 - Read 2th byte, OP R/M8, IMM8
      // 81 - Read 2th byte, OP R/M16, IMM16
      // ADD, -, ADC, SBB, -, SUB, -, CMP
      // 82 - Read 2th byte, OP R/M8, IMM8
      // 83 - Read 2th byte, OP R/M16, IMM8
      // 84 - TEST
      // 85 - TEST
      // 86 - XCHG
      // 87 - XCHG
      match first & 0x07 {
        0 => {
          let second = iter.next()?;
          let mod_rm = parseModRmByte(second, iter);
          let src_dest = OpBinarySrcDestDual::BYTE(
            OpBinarySrcDestByte::IMM_RM(mod_rm, iter.next()?));
          match (second >> 3) & 0x07 {
            0 => Op::ADD(src_dest),
            1 => Op::OR(src_dest),
            2 => Op::ADC(src_dest),
            3 => Op::SBB(src_dest),
            4 => Op::AND(src_dest),
            5 => Op::SUB(src_dest),
            6 => Op::XOR(src_dest),
            7 => Op::CMP(src_dest),
          }
        },
        1 => {
          let second = iter.next()?;
          let mod_rm = parseModRmWord(second, iter);
          let src_dest = OpBinarySrcDestDual::Word(
            OpBinarySrcDestWord::IMM_RM(mod_rm, iterNextU16(iter)?));
          match (second >> 3) & 0x07 {
            0 => Op::ADD(src_dest),
            1 => Op::OR(src_dest),
            2 => Op::ADC(src_dest),
            3 => Op::SBB(src_dest),
            4 => Op::AND(src_dest),
            5 => Op::SUB(src_dest),
            6 => Op::XOR(src_dest),
            7 => Op::CMP(src_dest),
          }
        },
        2 => {
          let second = iter.next()?;
          let mod_rm = parseModRmByte(second, iter);
          let src_dest = OpBinarySrcDestDual::BYTE(
            OpBinarySrcDestByte::IMM_RM(mod_rm, iter.next()?));
          match (second >> 3) & 0x07 {
            0 => Op::ADD(src_dest),
            1 => return None(),
            2 => Op::ADC(src_dest),
            3 => Op::SBB(src_dest),
            4 => return None(),
            5 => Op::SUB(src_dest),
            6 => return None(),
            7 => Op::CMP(src_dest),
          }
        },
        3 => {
          let second = iter.next()?;
          let mod_rm = parseModRmWord(second, iter);
          let src_dest = OpBinarySrcDestDual::Word(
            OpBinarySrcDestWord::IMM_RM(mod_rm, iterNextU16(iter)?));
          match (second >> 3) & 0x07 {
            0 => Op::ADD(src_dest),
            1 => return None(),
            2 => Op::ADC(src_dest),
            3 => Op::SBB(src_dest),
            4 => return None(),
            5 => Op::SUB(src_dest),
            6 => return None(),
            7 => Op::CMP(src_dest),
          }
        },
        4 => {
          // TEST
          let second = iter.next()?;
          Op::TEST(OpModRegRmDual::BYTE(parseModRegRmByte(second, iter)?))
        },
        5 => {
          // TEST
          let second = iter.next()?;
          Op::TEST(OpModRegRmDual::WORD(parseModRegRmWord(second, iter)?))
        },
        6 => {
          // XCHG
          let second = iter.next()?;
          Op::XCHG_RM_REG(OpModRegRmDual::BYTE(parseModRegRmByte(second, iter)?))
        },
        7 => {
          // XCHG
          let second = iter.next()?;
          Op::XCHG_RM_REG(OpModRegRmDual::WORD(parseModRegRmWord(second, iter)?))
        },
      }
    },
    0x88 => {
      // 88 - MOV
      // 89 - MOV
      // 8A - MOV
      // 8B - MOV
      // 8C - MOV
      // 8D - LEA
      // 8E - MOV
      // 8F - POP
      match first & 0x07 {
        0..=3 => {
          // MOV
          Op::MOV(parseBinarySrcDest(first, iter)?)
        },
        4 => {
          // mov r/m16, segreg
          let second = iter.next()?;
          let mod_rm = parseModRmWord(second, iter)?;
          let reg = parseSegmentRegister((second >> 3) & 0x07)?;
          Op::MOV_WORD_SEG(OpDirectionType::REG_TO_RM, mod_rm, reg)
        },
        5 => {
          // lea reg16, r/m16
          let second = iter.next()?;
          Op::LEA(parseModRegRmWord(second, iter)?)
        },
        6 => {
          // mov segreg, r/m16
          let second = iter.next()?;
          let mod_rm = parseModRmWord(second, iter)?;
          let reg = parseSegmentRegister((second >> 3) & 0x07)?;
          Op::MOV_WORD_SEG(OpDirectionType::RM_TO_REG, mod_rm, reg)
        },
        7 => {
          // pop r/m16 (second 000)
          let second = iter.next()?;
          let mod_rm = parseModRmWord(second, iter)?;
          match (second >> 3) & 0x07 {
            0 => Op::POP_RM(mod_rm),
            _ => return None(),
          }
        },
      }
    },
    0x90 => {
      // 90 - NOP
      // 91..97 - XCHG
      Op::XCHG_REG_AX(parseRegisterWord(first & 0x07))
    },
    0x98 => {
      // 98 - CBW
      // 99 - CWD
      // 9A - CALL FAR_PROC
      // 9B - WAIT
      // 9C - PUSHF
      // 9D - POPF
      // 9E - SAHF
      // 9F - LAHF
      match first & 0x07 {
        0 => Op::CBW(),
        1 => Op::CWD(),
        2 => Op::CALL_INTER_DIRECT(iterNextU16(iter)?, iterNextU16(iter)?),
        3 => Op::WAIT(),
        4 => Op::PUSHF(),
        5 => OP::POPF(),
        6 => Op::SAHF(),
        7 => Op::LAHF(),
      }
    },
    0xA0 => {
      // A0..A3 - MOV
      // A4..A5 - MOVS
      // A6..A7 - CMPS
      match first & 0x07 {
        0 => Op::MOV_MEM_TO_AL(iterNextU16(iter)?),
        1 => Op::MOV_MEM_TO_AX(iterNextU16(iter)?),
        2 => Op::MOV_AL_TO_MEM(iterNextU16(iter)?),
        3 => Op::MOV_AX_TO_MEM(iterNextU16(iter)?),
        4 => Op::MOVS(OpWordByte::BYTE),
        5 => Op::MOVS(OpWordByte::WORD),
        6 => Op::CMPS(OpWordByte::BYTE),
        7 => Op::CMPS(OpWordByte::WORD),
      }
    },
    0xA8 => {
      // A8..A9 - TEST
      // AA..AB - STOS
      // AC..AD - LODS
      // AE..AF - SCAS
      match first & 0x07 {
        0 => Op::TEST(OpBinarySrcDestDual::BYTE(OpBinarySrcDestByte::IMM_AX(
          iter.next()?))),
        1 => Op::TEST(OpBinarySrcDestDual::WORD(OpBinarySrcDestWord::IMM_AX(
          iterNextU16(iter)?))),
        2 => Op::STOS(OpWordByte::BYTE),
        3 => Op::STOS(OpWordByte::WORD),
        4 => Op::LODS(OpWordByte::BYTE),
        5 => Op::LODS(OpWordByte::WORD),
        6 => Op::SCAS(OpWordByte::BYTE),
        7 => Op::SCAS(OpWordByte::WORD),
      }
    },
    0xB0 => {
      // MOV
      Op::MOV(OpBinarySrcDestDual::BYTE(OpBinarySrcDestByte::IMM_REG(
        parseRegisterByte(first & 0x07))))
    },
    0xB8 => {
      // MOV
      Op::MOV(OpBinarySrcDestDual::WORD(OpBinarySrcDestWord::IMM_REG(
        parseRegisterWord(first & 0x07))))
    },
    0xC0 => {
      // C0 - 
      // C1 - 
      // C2..C3 - RET
      // C4 - LES
      // C5 - LDS
      // C6 - MOV
      // C7 - MOV
      match first & 0x07 {
        0 => return None(),
        1 => return None(),
        2 => Op::RET_INTRA_IMM(iterNextU16(iter)?),
        3 => Op::RET_INTRA(),
        4 => Op::LES(parseModRegRmWord(iter.next()?, iter)?),
        5 => Op::LDS(parseModRegRmWord(iter.next()?, iter)?),
        6 => {
          let second = iter.next()?;
          let mod_rm = parseModRmByte(second, iter)?;
          match (second >> 3) & 0x07 {
            0 => Op::MOV(OpBinarySrcDestDual::BYTE(
              OpBinarySrcDestByte::IMM_RM(mod_rm, iter.next()?))),
            _ => return None(),
          }
        },
        7 => {
          let second = iter.next()?;
          let mod_rm = parseModRmWord(second, iter)?;
          match (second >> 3) & 0x07 {
            0 => Op::MOV(OpBinarySrcDestDual::WORD(
              OpBinarySrcDestWORD::IMM_RM(mod_rm, iterNextU16(iter)?))),
            _ => return None(),
          }
        },
      }
    },
    0xC8 => {
      // C8 - 
      // C9 - 
      // CA - RET
      // CB - RET
      // CC - INT
      // CD - INT
      // CE - INTO
      // CF - IRET
      match first & 0x07 {
        0 => return None(),
        1 => return None(),
        2 => Op::RET_INTER_IMM(iterNextU16(iter)?),
        3 => Op::RET_INTER(),
        4 => Op::INT(3),
        5 => Op::INT(iter.next()?),
        6 => Op::INTO(),
        7 => Op::IRET(),
      }
    },
    0xD0 => {
      // ROL, ROR, RCL, RCR, SHL, SHR, -, SAR
      // D0 - op R/M8, 1
      // D1 - op R/M16, 1
      // D2 - op R/M8, CL
      // D3 - op R/M16, CL
      // D4 - AAM
      // D5 - AAD
      // D6 - 
      // D7 - XLAT
      match first & 0x07 {
        0..=3 => {
          let second = iter.next()?;
          let rotate_type = match (first >> 1) & 0x01 {
            0 => OpRotateType::ONE(),
            1 => OpRotateType::CL(),
          }
          let rm = match first & 0x01 {
            0 => OpModRmDual::BYTE(parseModRmByte(second, iter)?),
            1 => OpModRmDual::WORD(parseModRmWord(second, iter)?),
          };
          match (second >> 3) & 0x07 {
            0 => Op::ROL(rotate_type, rm),
            1 => Op::ROR(rotate_type, rm),
            2 => Op::RCL(rotate_type, rm),
            3 => Op::RCR(rotate_type, rm),
            4 => Op::SHL(rotate_type, rm),
            5 => Op::SHR(rotate_type, rm),
            6 => return None(),
            7 => Op::SAR(rotate_type, rm),
          }
        },
        4 => {
          let second = iter.next()?;
          if second == 0x0A {
            Op::AAM()
          } else {
            return None();
          }
        }
        5 => {
          let second = iter.next()?;
          if second == 0x0A {
            Op::AAD()
          } else {
            return None();
          }
        }
        6 => return None(),
        7 => Op::XLAT(),
      }
    },
    0xD8 => {
      // ESC
      let second = iter.next()?;
      let rm = parseModRmWord(second, iter)?;
      let esc_id = ((first & 0x7) << 3) + ((second >> 3) & 0x7);
      Op::ESC(esc_id, rm)
    },
    0xE0 => {
      // E0 - LOOPNE
      // E1 - LOOPE
      // E2 - LOOP
      // E3 - JCXZ
      // E4..E5 - IN
      // E6..E7 - OUT
      match first & 0x07 {
        0 => Op::LOOPNZ(iter.next()?),
        1 => Op::LOOPZ(iter.next()?),
        2 => Op::LOOP(iter.next()?),
        3 => Op::JCXZ(iter.next()?),
        4 => Op::IN_VARIABLE(OpWordByte::BYTE, iter.next()?),
        5 => Op::IN_VARIABLE(OpWordByte::WORD, iter.next()?),
        6 => Op::OUT_VARIABLE(OpWordByte::BYTE, iter.next()?),
        7 => Op::OUT_VARIABLE(OpWordByte::WORD, iter.next()?),
      }
    },
    0xE8 => {
      // E8 - CALL
      // E9 - JMP
      // EA - JMP
      // EB - JMP
      // EC..ED - IN
      // EE..EF - OUT
      match first & 0x07 {
        0 => Op::CALL_WITHIN_DIRECT(iterNextU16(iter)?),
        1 => Op::JMP_WITHIN_DIRECT(iterNextU16(iter)?),
        2 => Op::JMP_INTER_DIRECT(iterNextU16(iter)?, iterNextU16(iter)?),
        3 => Op::JMP_WITHIN_DIRECT_SHORT(iter.next()?),
        4 => Op::IN_FIXED(OpWordByte::BYTE),
        5 => Op::IN_FIXED(OpWordByte::WORD),
        6 => Op::OUT_FIXED(OpWordByte::BYTE),
        7 => Op::OUT_FIXED(OpWordByte::WORD),
      }
    },
    0xF0 => {
      // F0 - LOCK
      // F1 -
      // F2 - REPNE
      // F3 - REP
      // F4 - HLT
      // F5 - CMC
      // TEST, -, NOT, NEG, MUL, IMUL, DIV, IDIV
      // F6 - op R/M8
      // F7 - op R/M16
      match first & 0x07 {
        0 => Op::LOCK(),
        1 => return None(),
        2 => Op::REP_UNSET(),
        3 => Op::REP_SET(),
        4 => Op::HLT(),
        5 => Op::CMC(),
        6..=7 => {
          let second = iter.next()?;
          let rm = match first & 0x01 {
            0 => OpModRmDual::BYTE(parseModRmByte(second, iter)?),
            1 => OpModRmDual::WORD(parseModRmWord(second, iter)?),
          };
          match (second >> 3) & 0x07 {
            0 => match rm {
              OpModRmDual::BYTE(rm_raw) =>
                Op::TEST_IMM_BYTE(rm_raw, iter.next()?),
              OpModRmDual::WORD(rm_raw) =>
                Op::TEST_IMM_WORD(rm_raw, iterNextU16(iter)?),
            },
            1 => return None(),
            2 => Op::NOT(rm),
            3 => Op::NEG(rm),
            4 => Op::MUL(rm),
            5 => Op::IMUL(rm),
            6 => Op::DIV(rm),
            7 => Op::IDIV(rm),
          }
        },
      }
    },
    0xF8 => {
      // F8 - CLC
      // F9 - STC
      // FA - CLI
      // FB - STI
      // FC - CLD
      // FD - STD
      // INC, DEC, -, -, -, -, -, -
      // FE - op R/M8
      // INC, DEC, CALL, CALL, JMP, JMP, PUSH, -
      // FF - op MEM16
      match first & 0x07 {
        0 => Op::CLC(),
        1 => Op::STC(),
        2 => Op::CLI(),
        3 => Op::STI(),
        4 => Op::CLD(),
        5 => Op::STD(),
        6 => {
          let second = iter.next()?;
          let rm = parseModRmByte(second, iter)?;
          match (second >> 3) & 0x07 {
            0 => Op::INC_RM(OpModRmDual::BYTE(rm)),
            1 => Op::DEC_RM(OpModRmDual::BYTE(rm)),
            _ => return None(),
          }
        },
        7 => {
          let second = iter.next()?;
          let rm = parseModRmWord(second, iter)?;
          match (second >> 3) & 0x07 {
            0 => Op::INC_RM(OpModRmDual::WORD(rm)),
            1 => Op::DEC_RM(OpModRmDual::WORD(rm)),
            2 => Op::CALL_WITHIN_INDIRECT(rm),
            3 => Op::CALL_INTER_INDIRECT(rm),
            4 => Op::JMP_WITHIN_INDIRECT(rm),
            5 => Op::JMP_INTER_INDIRECT(rm),
            6 => Op::PUSH_RM(rm),
            _ => return None(),
          }
        },
      }
    },
  })
}
