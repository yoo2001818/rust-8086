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
  DIRECT,
  BP,
  BX,
}

pub enum OpModRmByte {
  REGISTER(OpRegisterByte),
  ADDRESS(OpAddressType),
  ADDRESS_DISP_BYTE(OpAddressType, u8),
  ADDRESS_DISP_WORD(OpAddressType, u16),
}

pub enum OpModRmWord {
  REGISTER(OpRegisterWord),
  ADDRESS(OpAddressType),
  ADDRESS_DISP_BYTE(OpAddressType, u8),
  ADDRESS_DISP_WORD(OpAddressType, u16),
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

pub fn parseOp(iter: Iterator<u16>): Option<Op> {
  const first_word = match iter.next() {
    Some(val) => val,
    None => return None,
  };
  const first_low = (first_word & 0xff) as u8;
  match first_low & 0xf8 {
    0x00 => {
      // ADD, PUSH ES, POP ES
    }
    0x08 => {
      // OR, PUSH CS
    },
    0x10 => {
      // ADC, PUSH SS, POP SS
    },
    0x18 => {
      // SBB, PUSH DS, POP DS
    },
    0x20 => {
      // AND, SELECT ES, DAA
    },
    0x28 => {
      // SUB, SELECT CS, DAS
    },
    0x30 => {
      // XOR, SELECT SS, AAA
    },
    0x38 => {
      // CMP, SELECT DS, AAS
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

    },
    0xA0 => {

    },
    0xA8 => {

    },
    0xB0 => {

    },
    0xB8 => {

    },
    0xC0 => {

    },
    0xC8 => {

    },
    0xD0 => {

    },
    0xD8 => {

    },
    0xE0 => {

    },
    0xE8 => {

    },
    0xF0 => {

    },
    0xF8 => {

    },
  }
}
