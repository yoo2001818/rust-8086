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

pub enum OpAddress {
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

pub enum OpRmByte {
  REGISTER(OpRegisterByte),
  ADDRESS(OpAddress),
  ADDRESS_DISP_BYTE(OpAddress, u8),
  ADDRESS_DISP_WORD(OpAddress, u16),
}

pub enum OpRmWord {
  REGISTER(OpRegisterWord),
  ADDRESS(OpAddress),
  ADDRESS_DISP_BYTE(OpAddress, u8),
  ADDRESS_DISP_WORD(OpAddress, u16),
}

pub struct OpModRm {
  register: OpMemoryByte,
  rm: OpRmByte,
}

pub struct OpModRmWord {
  register: OpMemoryWord,
  rm: OpRmWord,
}

pub enum Op {
  MOV_WORD_REG_TO_RM(OpModRmWord),
  MOV_BYTE_REG_TO_RM(OpModRmByte),
  MOV_WORD_RM_TO_REG(OpModRmWord),
  MOV_BYTE_RM_TO_REG(OpModRmByte),
  MOV_WORD_IMM_TO_RM(OpRmWord, i16),
  MOV_BYTE_IMM_TO_RM(OpRmByte, i8),
  MOV_WORD_IMM_TO_REG(OpRegisterWord, u16),
  MOV_BYTE_IMM_TO_REG(OpRegisterByte, u8),
  MOV_WORD_RM_TO_SEG(OpRmWord, OpSegmentRegister),
  MOV_WORD_SEG_TO_RM(OpSegmentRegister, OpRmWord),
  PUSH_RM(OpRmWord),
  PUSH_REG(OpRegisterWord),
  PUSH_SEG(OpSegmentRegister),
  POP_RM(OpRmWord),
  POP_REG(OpRegisterWord),
  POP_SEG(OpSegmentRegister),
  XCHG_WORD_RM_REG(OpModRmWord),
  XCHG_BYTE_RM_REG(OpModRmByte),
  XCHG_REG_AX(OpRegisterWord),
  IN_WORD_FIXED(),
  IN_BYTE_FIXED(),
  IN_WORD_VARIABLE(),
  IN_BYTE_VARIABLE(),
  OUT(),
  XLAT(),
  LEA(),
  LDS(),
  LES(),
  LAHF(),
  SAHF(),
  PUSHF(),
  POPF(),
  ADD(),
  ADC(),
  INC(),
  AAA(),
  DAA(),
  SUB(),
  SBB(),
  DEC(),
  NEG(),
}
