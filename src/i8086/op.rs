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

pub enum OpModRmByte {
  REGISTER(OpRegisterByte),
  ADDRESS(OpAddressType),
  ADDRESS_DISP_BYTE(OpAddressType, u8),
  ADDRESS_DISP_WORD(OpAddressType, u16),
  DIRECT(u16),
}

pub enum OpModRmWord {
  REGISTER(OpRegisterWord),
  ADDRESS(OpAddressType),
  ADDRESS_DISP_BYTE(OpAddressType, u8),
  ADDRESS_DISP_WORD(OpAddressType, u16),
  DIRECT(u16),
}

pub enum OpModRm {
  BYTE(OpModRmByte),
  WORD(OpModRmWord),
}

pub enum OpDirectionType {
  REG_TO_RM,
  RM_TO_REG,
}

pub struct OpModRegRmByte {
  direction: OpDirectionType,
  register: OpRegisterByte,
  rm: OpModRmByte,
}

pub struct OpModRegRmWord {
  direction: OpDirectionType,
  register: OpRegisterWord,
  rm: OpModRmWord,
}

pub enum OpModRegRm {
  BYTE(OpModRegRmByte),
  WORD(OpModRegRmWord),
}

pub enum OpBinarySrcDestWord {
  REG_RM(OpModRegRmWord),
  IMM_RM(OpModRmWord, i16),
  IMM_REG(OpRegisterWord, i16),
  IMM_AX(i16),
}

pub enum OpBinarySrcDestByte {
  REG_RM(OpModRegRmByte),
  IMM_RM(OpModRmByte, i8),
  IMM_REG(OpRegisterByte, i8),
  IMM_AL(i8),
}

pub enum OpBinarySrcDest {
  BYTE(OpBinarySrcDestByte),
  WORD(OpBinarySrcDestWord),
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
  MOV(OpBinaryDest),
  MOV_WORD_SEG(OpDirectionType, OpModRmWord, OpSegmentRegister),
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
  ADD(OpBinarySrcDest),
  ADC(OpBinarySrcDest),
  INC_RM(OpModRm),
  INC_REG(OpRegisterWord),
  AAA,
  DAA,
  SUB(OpBinarySrcDest),
  SBB(OpBinarySrcDest),
  DEC_RM(OpModRm),
  DEC_REG(OpRegisterWord),
  NEG(OpModRm),
  CMP(OpBinarySrcDest),
  AAS,
  DAS,
  MUL(OpModRm),
  IMUL(OpModRm),
  AAM,
  DIV(OpModRm),
  IDIV(OpModRm),
  AAD,
  CSW,
  CWD,
  NOT(OpModRm),
  SHL(OpRotateType, OpModRm),
  SHR(OpRotateType, OpModRm),
  SAR(OpRotateType, OpModRm),
  ROL(OpRotateType, OpModRm),
  ROR(OpRotateType, OpModRm),
  RCL(OpRotateType, OpModRm),
  RCR(OpRotateType, OpModRm),
  AND(OpBinarySrcDest),
  TEST(OpBinarySrcDest),
  OR(OpBinarySrcDest),
  XOR(OpBinarySrcDest),
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
  ESC(u8, u8, OpModRmWord),
  LOCK,
  SEGMENT(OpSegmentRegister),
}

pub fn parseRegisterWord(byte: u8): OpRegisterWord {
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

pub fn parseRegisterByte(byte: u8): OpRegisterByte {
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

pub fn parseAddressType(byte: u8): OpAddressType {
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

fn iterNextU16 (iter: &mut Iterator<u8>): u16 {
  iter.next().unwrap() as u16 +
    ((iter.next().unwrap() as u16) << 8)
}

pub fn parseModRmWord(byte: u8, iter: &mut Iterator<u8>): OpModRmWord {
  let mod_val = (byte >> 5) & 0x07;
  let rm_val = byte & 0x03;
  match rm_val {
    0 => OpModRmWord::REGISTER(parseRegisterWord(mod_val)),
    1 => {
      let addr_type = parseAddressType(mod_val);
      if addr_type == OpAddressType::BP {
        OpModRmWord::DIRECT(iterNextU16(iter))
      } else {
        OpModRmWord::ADDRESS(addr_type)
      }
    }
    2 => OpModRmWord::ADDRESS_DISP_BYTE(parseAddressType(mod_val),
      iter.next().unwrap()),
    3 => OpModRmWord::ADDRESS_DISP_WORD(parseAddressType(mod_val),
      iterNextU16(iter))
  }
}

pub fn parseModRmByte(byte: u8, iter: &mut Iterator<u8>): OpModRmByte {
  let mod_val = (byte >> 5) & 0x07;
  let rm_val = byte & 0x03;
  match rm_val {
    0 => OpModRmByte::REGISTER(parseRegisterByte(mod_val)),
    1 => {
      let addr_type = parseAddressType(mod_val);
      if addr_type == OpAddressType::BP {
        OpModRmByte::DIRECT(
          iter.next().unwrap() as u16 +
          ((iter.next().unwrap() as u16) << 8),
        )
      } else {
        OpModRmByte::ADDRESS(addr_type)
      }
    }
    2 => OpModRmByte::ADDRESS_DISP_BYTE(parseAddressType(mod_val),
      iter.next().unwrap()),
    3 => OpModRmByte::ADDRESS_DISP_WORD(parseAddressType(mod_val),
      iter.next().unwrap() as u16 +
      ((iter.next().unwrap() as u16) << 8)),
  }
}

pub fn parseBinarySrcDest(
  first: u8, iter: &mut Iterator<u8>,
): OpBinarySrcDest {
  match first & 0x07 {
    0..=3 => {
      let second = iter.next().unwrap();
      let direction = match first & 0x02 {
        0 => OpDirectionType::REG_TO_RM,
        2 => OpDirectionType::RM_TO_REG,
        _ => panic!("This should never happen"),
      }
      match first & 0x01 {
        0 => OpBinarySrcDest::BYTE(OpBinarySrcDestByte::REG_RM(
          OpModRegRmByte {
            direction: direction,
            register: parseRegisterByte((second >> 3) & 0x7),
            rm: parseModRmByte(second, iter),
          }
        )),
        1 => OpBinarySrcDest::WORD(OpBinarySrcDestWord::REG_RM(
          OpModRegRmWord {
            direction: direction,
            register: parseRegisterByte((second >> 3) & 0x7),
            rm: parseModRmByte(second, iter),
          }
        )),
        _ => panic!("This should never happen"),
      }
    },
    4 => OpBinarySrcDest::BYTE(
      OpBinarySrcDestByte::IMM_AL(iter.next().unwrap())),
    5 => OpBinarySrcDest::WORD(
      OpBinarySrcDestWord::IMM_AX(iterNextU16(iter))),
    _ => panic!("..."),
  }
}

pub fn parseOp(iter: &mut Iterator<u8>): Option<Op> {
  const first = match iter.next() {
    Some(val) => val,
    None => return None,
  };
  match first & 0xf8 {
    0x00 => {
      // ADD, PUSH ES, POP ES
      match first & 0x07 {
        0..=5 => Op::ADD(parseBinarySrcDest(first, iter)),
        6 => Op::PUSH_SEG(OpSegmentRegister::ES),
        7 => Op::POP_SEG(OpSegmentRegister::ES),
        _ => panic!(""),
      }
    }
    0x08 => {
      // OR, PUSH CS
      match first & 0x07 {
        0..=5 => Op::OR(parseBinarySrcDest(first, iter)),
        6 => Op::PUSH_SEG(OpSegmentRegister::CS),
        7 => Op::POP_SEG(OpSegmentRegister::CS),
        _ => panic!(""),
      }
    },
    0x10 => {
      // ADC, PUSH SS, POP SS
      match first & 0x07 {
        0..=5 => Op::ADC(parseBinarySrcDest(first, iter)),
        6 => Op::PUSH_SEG(OpSegmentRegister::SS),
        7 => Op::POP_SEG(OpSegmentRegister::SS),
        _ => panic!(""),
      }
    },
    0x18 => {
      // SBB, PUSH DS, POP DS
      match first & 0x07 {
        0..=5 => Op::SBB(parseBinarySrcDest(first, iter)),
        6 => Op::PUSH_SEG(OpSegmentRegister::DS),
        7 => Op::POP_SEG(OpSegmentRegister::DS),
        _ => panic!(""),
      }
    },
    0x20 => {
      // AND, SELECT ES, DAA
      match first & 0x07 {
        0..=5 => Op::AND(parseBinarySrcDest(first, iter)),
        6 => Op::SEGMENT(OpSegmentRegister::ES),
        7 => Op::DAA,
        _ => panic!(""),
      }
    },
    0x28 => {
      // SUB, SELECT CS, DAS
      match first & 0x07 {
        0..=5 => Op::SUB(parseBinarySrcDest(first, iter)),
        6 => Op::SEGMENT(OpSegmentRegister::CS),
        7 => Op::DAS,
        _ => panic!(""),
      }
    },
    0x30 => {
      // XOR, SELECT SS, AAA
      match first & 0x07 {
        0..=5 => Op::XOR(parseBinarySrcDest(first, iter)),
        6 => Op::SEGMENT(OpSegmentRegister::SS),
        7 => Op::AAA,
        _ => panic!(""),
      }
    },
    0x38 => {
      // CMP, SELECT DS, AAS
      match first & 0x07 {
        0..=5 => Op::CMP(parseBinarySrcDest(first, iter)),
        6 => Op::SEGMENT(OpSegmentRegister::DS),
        7 => Op::AAS,
        _ => panic!(""),
      }
    },
    0x40 => {
      // INC
    },
    0x48 => {
      // DEC
    },
    0x50 => {
      // PUSH
    },
    0x58 => {
      // POP
    },
    0x60 => {
      // not used
    },
    0x68 => {
      // not used
    },
    0x70 => {
      // JO, JNO, JB, JNB, JE, JNE, JBE, JNBE
    },
    0x78 => {
      // JS, JNS, JP, JNP, JL, JNL, JLE, JNLE
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
    },
    0x90 => {
      // 90 - NOP
      // 91..97 - XCHG
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
    },
    0xA0 => {
      // A0..A3 - MOV
      // A4..A5 - MOVS
      // A6..A7 - CMPS
    },
    0xA8 => {
      // A8..A9 - TEST
      // AA..AB - STOS
      // AC..AD - LODS
      // AE..AF - SCAS
    },
    0xB0 => {
      // MOV
    },
    0xB8 => {
      // MOV
    },
    0xC0 => {
      // C0 - 
      // C1 - 
      // C2..C3 - RET
      // C4 - LES
      // C5 - LDS
      // C6 - MOV
      // C7 - MOV
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
    },
    0xD8 => {
      // ESC
    },
    0xE0 => {
      // E0 - LOOPNE
      // E1 - LOOPE
      // E2 - LOOP
      // E3 - JCXZ
      // E4..E5 - IN
      // E6..E7 - OUT
    },
    0xE8 => {
      // E8 - CALL
      // E9 - JMP
      // EA - JMP
      // EB - JMP
      // EC..ED - IN
      // EE..EF - OUT
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
    },
  }
}
