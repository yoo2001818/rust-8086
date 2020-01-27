
#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpRegister {
  Ax,
  Cx,
  Dx,
  Bx,
  Sp,
  Bp,
  Si,
  Di,
  Al,
  Cl,
  Dl,
  Bl,
  Ah,
  Ch,
  Dh,
  Bh,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpSegmentRegister {
  Es,
  Cs,
  Ss,
  Ds,
}


#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpAddressType {
  BxSi,
  BxDi,
  BpSi,
  BpDi,
  Si,
  Di,
  Bp,
  Bx,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpTarget {
  Register(OpRegister),
  SegmentRegister(OpSegmentRegister),
  Address(OpAddressType, u16),
  Direct(u16),
  ImmWord(u16),
  ImmByte(u8),
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpShiftType {
  One,
  Cl,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpSize {
  Byte,
  Word,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpCallType {
  WithinDirect(u16),
  WithinIndirect(OpTarget),
  InterDirect(u16, u16),
  InterIndirect(OpTarget),
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpBinaryOp {
  // Immed
  Add,
  Or,
  Adc,
  Sbb,
  And,
  Sub,
  Xor,
  Cmp,
  Xchg,
  Test,
  Mov,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpUnaryOp {
  Push,
  Pop,
  Inc,
  Dec,
  Not,
  Neg,
  Mul,
  Imul,
  Div,
  Idiv,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpShiftOp {
  Rol,
  Ror,
  Rcl,
  Rcr,
  Shl,
  Shr,
  Sal,
  Sar,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpCondJmpOp {
  Jo,
  Jno,
  Js,
  Jns,
  Je, // Jz
  Jne, // Jnz
  Jb, // Jnae, Jc
  Jnb, // Jae, Jnc
  Jbe, // Jna
  Ja, // Jnbe
  Jl, // Jnge
  Jge, // Jnl
  Jle, // Jng
  Jg, // Jnle
  Jp, // Jpe
  Jnp, // Jpo
  Jcxz, // Jecxz
  Loopne, // Loopnz
  Loope, // Loopz
  Loop,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpStringOp {
  Movs(OpSize),
  Cmps(OpSize),
  Scas(OpSize),
  Lods(OpSize),
  Stos(OpSize),
  Rep,
  Repz,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Op {
  Binary(OpBinaryOp, OpSize, OpTarget, OpTarget),
  Unary(OpUnaryOp, OpSize, OpTarget),
  CondJmp(OpCondJumpOp, u8),
  Str(OpStringOp),
  InFixed(OpSize),
  InVariable(OpSize, u8),
  OutFixed(OpSize),
  OutVariable(OpSize, u8),
  Xlat,
  Lea(OpRegister, OpTarget),
  Lds(OpRegister, OpTarget),
  Les(OpRegister, OpTarget),
  Lahf,
  Sahf,
  Pushf,
  Popf,
  Aaa,
  Daa,
  Aas,
  Das,
  Aam,
  Aad,
  Cbw,
  Cwd,
  Call(OpCallType),
  Jmp(OpCallType),
  RetWithin,
  RetWithinImm(u16),
  RetInter,
  RetInterImm(u16),
  Int(u8),
  Into,
  Iret,
  Clc,
  Cmc,
  Stc,
  Cld,
  Std,
  Cli,
  Sti,
  Hlt,
  Wait,
  Esc(u8, OpModRm),
  Lock,
  Segment(OpSegmentRegister),
}
