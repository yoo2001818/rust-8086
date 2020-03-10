use super::register::*;
use super::operand::*;

trait RegisterModRmParsable: Sized {
  fn from_value(value: u8) -> Option<Self>;
}

impl RegisterModRmParsable for RegisterWordType {
  fn from_value(value: u8) -> Option<RegisterWordType> {
    Some(match value {
      0 => RegisterWordType::Ax,
      1 => RegisterWordType::Cx,
      2 => RegisterWordType::Dx,
      3 => RegisterWordType::Bx,
      4 => RegisterWordType::Sp,
      5 => RegisterWordType::Bp,
      6 => RegisterWordType::Si,
      7 => RegisterWordType::Di,
      _ => return None,
    })
  }
}

impl RegisterWordType {
  fn from_seg(value: u8) -> Option<RegisterWordType> {
    Some(match value {
      0 => RegisterWordType::Es,
      1 => RegisterWordType::Cs,
      2 => RegisterWordType::Ss,
      3 => RegisterWordType::Ds,
      _ => return None,
    })
  }
}

impl RegisterModRmParsable for RegisterByteType {
  fn from_value(value: u8) -> Option<RegisterByteType> {
    Some(match value {
      0 => RegisterByteType::Al,
      1 => RegisterByteType::Cl,
      2 => RegisterByteType::Dl,
      3 => RegisterByteType::Bl,
      4 => RegisterByteType::Ah,
      5 => RegisterByteType::Ch,
      6 => RegisterByteType::Dh,
      7 => RegisterByteType::Bh,
      _ => return None,
    })
  }
}

impl AddressType {
  fn from_value(value: u8) -> Option<AddressType> {
    Some(match value {
      0 => AddressType::BxSi,
      1 => AddressType::BxDi,
      2 => AddressType::BpSi,
      3 => AddressType::BpDi,
      4 => AddressType::Si,
      5 => AddressType::Di,
      6 => AddressType::Bp,
      7 => AddressType::Bx,
      _ => return None,
    })
  }
  fn to_value(&self) -> u8 {
    *self as u8
  }
}

#[derive(PartialEq, Copy, Clone)]
#[derive(Debug)]
pub enum OpShiftType {
  One,
  Cl,
}

#[derive(PartialEq, Copy, Clone)]
#[derive(Debug)]
pub enum OpSize {
  Byte,
  Word,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpCallType {
  WithinDirect(u16),
  WithinIndirect(OperandWord),
  InterDirect(u16, u16),
  InterIndirect(OperandWord),
}

#[derive(PartialEq, Copy, Clone)]
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

impl OpBinaryOp {
  fn from_immed(value: u8) -> Option<OpBinaryOp> {
    Some(match value {
      0 => OpBinaryOp::Add,
      1 => OpBinaryOp::Or,
      2 => OpBinaryOp::Adc,
      3 => OpBinaryOp::Sbb,
      4 => OpBinaryOp::And,
      5 => OpBinaryOp::Sub,
      6 => OpBinaryOp::Xor,
      7 => OpBinaryOp::Cmp,
      _ => return None,
    })
  }
}

#[derive(PartialEq, Copy, Clone)]
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

impl OpUnaryOp {
  fn from_grp1(value: u8) -> Option<OpUnaryOp> {
    Some(match value {
      0 => return None,
      1 => return None,
      2 => OpUnaryOp::Not,
      3 => OpUnaryOp::Neg,
      4 => OpUnaryOp::Mul,
      5 => OpUnaryOp::Imul,
      6 => OpUnaryOp::Div,
      7 => OpUnaryOp::Idiv,
      _ => return None,
    })
  }
  fn from_grp2(value: u8) -> Option<OpUnaryOp> {
    Some(match value {
      0 => OpUnaryOp::Inc,
      1 => OpUnaryOp::Dec,
      6 => OpUnaryOp::Push,
      _ => return None,
    })
  }
}

#[derive(PartialEq, Copy, Clone)]
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

impl OpShiftOp {
  fn from_value(value: u8) -> Option<OpShiftOp> {
    Some(match value {
      0 => OpShiftOp::Rol,
      1 => OpShiftOp::Ror,
      2 => OpShiftOp::Rcl,
      3 => OpShiftOp::Rcr,
      4 => OpShiftOp::Shl,
      5 => OpShiftOp::Shr,
      6 => OpShiftOp::Sal,
      7 => OpShiftOp::Sar,
      _ => return None,
    })
  }
}

#[derive(PartialEq, Copy, Clone)]
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

#[derive(PartialEq, Copy, Clone)]
#[derive(Debug)]
pub enum OpNullaryOp {
  Xlat,
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
  Rep,
  Repz,
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
  Lock,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Op {
  BinaryByte { op: OpBinaryOp, src: OperandByte, dest: OperandByte },
  BinaryWord { op: OpBinaryOp, src: OperandWord, dest: OperandWord },
  UnaryByte { op: OpUnaryOp, dest: OperandByte },
  UnaryWord { op: OpUnaryOp, dest: OperandWord },
  ShiftByte { op: OpShiftOp,shift_type: OpShiftType, dest: OperandByte },
  ShiftWord { op: OpShiftOp,shift_type: OpShiftType, dest: OperandWord },
  Nullary(OpNullaryOp),
  CondJmp { op: OpCondJmpOp, offset: i8 },
  InFixed(OpSize),
  InVariable(OpSize, u8),
  OutFixed(OpSize),
  OutVariable(OpSize, u8),
  Lea(RegisterWordType, OperandWord),
  Lds(RegisterWordType, OperandWord),
  Les(RegisterWordType, OperandWord),
  Movs(OpSize),
  Cmps(OpSize),
  Scas(OpSize),
  Lods(OpSize),
  Stos(OpSize),
  Call(OpCallType),
  Jmp(OpCallType),
  RetWithin,
  RetWithinImm(u16),
  RetInter,
  RetInterImm(u16),
  Int(u8),
  Esc(u8, OperandWord),
  Segment(RegisterWordType),
}

fn iter_next_u16(iter: &mut dyn Iterator<Item = u8>) -> Option<u16> {
  Some(iter.next()? as u16 +
    ((iter.next()? as u16) << 8))
}

fn parse_mod_rm<T: RegisterType + RegisterModRmParsable>(
  second: u8,
  iter: &mut dyn Iterator<Item = u8>,
) -> Option<Operand<T>> {
  let mod_val = (second >> 6) & 0x03;
  let rm_val = second & 0x07;
  Some(match mod_val {
    0 => {
      let addr_type = AddressType::from_value(rm_val)?;
      if addr_type == AddressType::Bp {
        Operand::Direct(iter_next_u16(iter)?)
      } else {
        Operand::Address(addr_type, 0)   
      }
    },
    1 => Operand::Address(
      AddressType::from_value(rm_val)?, iter.next()? as i8 as i16),
    2 => Operand::Address(
      AddressType::from_value(rm_val)?, iter_next_u16(iter)? as i16),
    3 => Operand::Register(T::from_value(rm_val)?),
    _ => return None,
  })
}

fn parse_reg<T: RegisterType + RegisterModRmParsable>(
  value: u8,
) -> Option<Operand<T>> {
  Some(Operand::Register(T::from_value(value)?))
}

fn create_binary_op_byte(
  second: u8,
  iter: &mut dyn Iterator<Item = u8>,
  op: OpBinaryOp,
  other: OperandByte,
  inversed: bool,
) -> Option<Op> {
  let mod_rm = parse_mod_rm::<RegisterByteType>(second, iter)?;
  if inversed {
    Some(Op::BinaryByte { op, src: other, dest: mod_rm })
  } else {
    Some(Op::BinaryByte { op, src: mod_rm, dest: other })
  }
}

fn create_binary_op_word(
  second: u8,
  iter: &mut dyn Iterator<Item = u8>,
  op: OpBinaryOp,
  other: OperandWord,
  inversed: bool,
) -> Option<Op> {
  let mod_rm = parse_mod_rm::<RegisterWordType>(second, iter)?;
  if inversed {
    Some(Op::BinaryWord { op, src: other, dest: mod_rm })
  } else {
    Some(Op::BinaryWord { op, src: mod_rm, dest: other })
  }
}

fn parse_binary_group_op(
  op: OpBinaryOp,
  first: u8,
  iter: &mut dyn Iterator<Item = u8>,
) -> Option<Op> {
  Some(match first & 0x07 {
    0 | 2 => {
      let second = iter.next()?;
      let reg = parse_reg::<RegisterByteType>((second >> 3) & 0x07)?;
      let inversed = first & 0x02 == 0;
      create_binary_op_byte(second, iter, op, reg, inversed)?
    }
    1 | 3 => {
      let second = iter.next()?;
      let reg = parse_reg::<RegisterWordType>((second >> 3) & 0x07)?;
      let inversed = first & 0x02 == 0;
      create_binary_op_word(second, iter, op, reg, inversed)?
    }
    4 => Op::BinaryByte {
      op: op,
      src: Operand::ImmByte(iter.next()?),
      dest: Operand::Register(RegisterByteType::Al),
    },
    5 => Op::BinaryWord {
      op: op,
      src: Operand::ImmWord(iter_next_u16(iter)?),
      dest: Operand::Register(RegisterWordType::Ax),
    },
    _ => return None,
  })
}

pub fn parse_op(iter: &mut dyn Iterator<Item = u8>) -> Option<Op> {
  let first = iter.next()?;
  let first_octet = first & 0x07;
  Some(match first & 0xf8 {
    0x00 => {
      match first_octet {
        0..=5 => parse_binary_group_op(OpBinaryOp::Add, first, iter)?,
        6 => Op::UnaryWord {
          op: OpUnaryOp::Push,
          dest: Operand::Register(RegisterWordType::Es),
        },
        7 => Op::UnaryWord {
          op: OpUnaryOp::Pop,
          dest: Operand::Register(RegisterWordType::Es),
        },
        _ => return None,
      }
    },
    0x08 => {
      match first_octet {
        0..=5 => parse_binary_group_op(OpBinaryOp::Or, first, iter)?,
        6 => Op::UnaryWord {
          op: OpUnaryOp::Push,
          dest: Operand::Register(RegisterWordType::Cs),
        },
        7 => Op::UnaryWord {
          op: OpUnaryOp::Pop,
          dest: Operand::Register(RegisterWordType::Cs),
        },
        _ => return None,
      }
    },
    0x10 => {
      match first_octet {
        0..=5 => parse_binary_group_op(OpBinaryOp::Adc, first, iter)?,
        6 => Op::UnaryWord {
          op: OpUnaryOp::Push,
          dest: Operand::Register(RegisterWordType::Ss),
        },
        7 => Op::UnaryWord {
          op: OpUnaryOp::Pop,
          dest: Operand::Register(RegisterWordType::Ss),
        },
        _ => return None,
      }
    },
    0x18 => {
      match first_octet {
        0..=5 => parse_binary_group_op(OpBinaryOp::Sbb, first, iter)?,
        6 => Op::UnaryWord {
          op: OpUnaryOp::Push,
          dest: Operand::Register(RegisterWordType::Ds),
        },
        7 => Op::UnaryWord {
          op: OpUnaryOp::Pop,
          dest: Operand::Register(RegisterWordType::Ds),
        },
        _ => return None,
      }
    },
    0x20 => {
      match first_octet {
        0..=5 => parse_binary_group_op(OpBinaryOp::And, first, iter)?,
        6 => Op::Segment(RegisterWordType::Es),
        7 => Op::Nullary(OpNullaryOp::Daa),
        _ => return None,
      }
    },
    0x28 => {
      match first_octet {
        0..=5 => parse_binary_group_op(OpBinaryOp::Sub, first, iter)?,
        6 => Op::Segment(RegisterWordType::Cs),
        7 => Op::Nullary(OpNullaryOp::Das),
        _ => return None,
      }
    },
    0x30 => {
      match first_octet {
        0..=5 => parse_binary_group_op(OpBinaryOp::Xor, first, iter)?,
        6 => Op::Segment(RegisterWordType::Ss),
        7 => Op::Nullary(OpNullaryOp::Aaa),
        _ => return None,
      }
    },
    0x38 => {
      match first_octet {
        0..=5 => parse_binary_group_op(OpBinaryOp::Cmp, first, iter)?,
        6 => Op::Segment(RegisterWordType::Ds),
        7 => Op::Nullary(OpNullaryOp::Aas),
        _ => return None,
      }
    },
    0x40 => {
      Op::UnaryWord {
        op: OpUnaryOp::Inc,
        dest: Operand::Register(RegisterWordType::from_value(first_octet)?),
      }
    },
    0x48 => {
      Op::UnaryWord {
        op: OpUnaryOp::Dec,
        dest: Operand::Register(RegisterWordType::from_value(first_octet)?),
      }
    },
    0x50 => {
      Op::UnaryWord {
        op: OpUnaryOp::Push,
        dest: Operand::Register(RegisterWordType::from_value(first_octet)?),
      }
    },
    0x58 => {
      Op::UnaryWord {
        op: OpUnaryOp::Pop,
        dest: Operand::Register(RegisterWordType::from_value(first_octet)?),
      }
    },
    0x60 => {
      // not used
      return None;
    },
    0x68 => {
      // not used
      return None;
    },
    0x70 => {
      let second = iter.next()?;
      let jmp_type = match first_octet {
        0 => OpCondJmpOp::Jo,
        1 => OpCondJmpOp::Jno,
        2 => OpCondJmpOp::Jb,
        3 => OpCondJmpOp::Jnb,
        4 => OpCondJmpOp::Je,
        5 => OpCondJmpOp::Jne,
        6 => OpCondJmpOp::Jbe,
        7 => OpCondJmpOp::Ja,
        _ => return None,
      };
      Op::CondJmp { op: jmp_type, offset: second as i8 }
    },
    0x78 => {
      let second = iter.next()?;
      let jmp_type = match first_octet {
        0 => OpCondJmpOp::Js,
        1 => OpCondJmpOp::Jns,
        2 => OpCondJmpOp::Jp,
        3 => OpCondJmpOp::Jnp,
        4 => OpCondJmpOp::Jl,
        5 => OpCondJmpOp::Jge,
        6 => OpCondJmpOp::Jle,
        7 => OpCondJmpOp::Jg,
        _ => return None,
      };
      Op::CondJmp { op: jmp_type, offset: second as i8 }
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
      match first_octet {
        0..=3 => {
          let second = iter.next()?;
          let op = OpBinaryOp::from_immed((second >> 3) & 0x07)?;
          match first & 0x01 {
            0 => {
              let mod_rm = parse_mod_rm(second, iter)?;
              let imm = Operand::ImmByte(iter.next()?);
              Op::BinaryByte {
                op,
                dest: mod_rm,
                src: imm,
              }
            },
            1 => {
              let mod_rm = parse_mod_rm(second, iter)?;
              let imm = match first & 0x03 {
                1 => Operand::ImmWord(iter_next_u16(iter)?),
                _ => Operand::ImmByte(iter.next()?),
              };
              Op::BinaryWord {
                op,
                dest: mod_rm,
                src: imm,
              }
            },
            _ => return None,
          }
        },
        4 => {
          let second = iter.next()?;
          Op::BinaryByte {
            op: OpBinaryOp::Test,
            src: parse_reg((second >> 3) & 0x07)?,
            dest: parse_mod_rm(second, iter)?,
          }
        },
        5 => {
          let second = iter.next()?;
          Op::BinaryWord {
            op: OpBinaryOp::Test,
            src: parse_reg((second >> 3) & 0x07)?,
            dest: parse_mod_rm(second, iter)?,
          }
        },
        6 => {
          let second = iter.next()?;
          Op::BinaryByte {
            op: OpBinaryOp::Xchg,
            src: parse_reg((second >> 3) & 0x07)?,
            dest: parse_mod_rm(second, iter)?,
          }
        },
        7 => {
          let second = iter.next()?;
          Op::BinaryWord {
            op: OpBinaryOp::Xchg,
            src: parse_reg((second >> 3) & 0x07)?,
            dest: parse_mod_rm(second, iter)?,
          }
        },
        _ => return None,
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
      // 8F - Pop
      match first_octet {
        0..=3 => {
          // MOV
          parse_binary_group_op(OpBinaryOp::Mov, first, iter)?
        },
        4 => {
          // MOV r/m16, segreg
          let second = iter.next()?;
          let reg = RegisterWordType::from_seg((second >> 3) & 0x07)?;
          Op::BinaryWord {
            op: OpBinaryOp::Mov,
            src: Operand::Register(reg),
            dest: parse_mod_rm(second, iter)?,
          }
        },
        5 => {
          // LEA reg16, r/m16
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(second, iter)?;
          let reg = RegisterWordType::from_value((second >> 3) & 0x07)?;
          Op::Lea(reg, mod_rm)
        },
        6 => {
          // MOV segreg, r/m16
          let second = iter.next()?;
          let reg = RegisterWordType::from_seg((second >> 3) & 0x07)?;
          Op::BinaryWord {
            op: OpBinaryOp::Mov,
            src: parse_mod_rm(second, iter)?,
            dest: Operand::Register(reg),
          }
        },
        7 => {
          // Pop r/m16 (second 000)
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(second, iter)?;
          match (second >> 3) & 0x07 {
            0 => Op::UnaryWord {
              op: OpUnaryOp::Pop,
              dest: mod_rm,
            },
            _ => return None,
          }
        },
        _ => return None,
      }
    },
    0x90 => {
      Op::BinaryWord {
        op: OpBinaryOp::Xchg,
        src: Operand::Register(RegisterWordType::Ax),
        dest: parse_reg(first_octet)?,
      }
    },
    0x98 => {
      match first_octet {
        0 => Op::Nullary(OpNullaryOp::Cbw),
        1 => Op::Nullary(OpNullaryOp::Cwd),
        2 => Op::Call(OpCallType::InterDirect(iter_next_u16(iter)?, iter_next_u16(iter)?)),
        3 => Op::Nullary(OpNullaryOp::Wait),
        4 => Op::Nullary(OpNullaryOp::Pushf),
        5 => Op::Nullary(OpNullaryOp::Popf),
        6 => Op::Nullary(OpNullaryOp::Sahf),
        7 => Op::Nullary(OpNullaryOp::Lahf),
        _ => return None,
      }
    },
    0xA0 => {
      // A0..A3 - MOV
      // A4..A5 - MOVS
      // A6..A7 - CMPS
      match first_octet {
        0 => Op::BinaryByte {
          op: OpBinaryOp::Mov,
          src: Operand::Direct(iter_next_u16(iter)?),
          dest: Operand::Register(RegisterByteType::Al),
        },
        1 => Op::BinaryWord {
          op: OpBinaryOp::Mov,
          src: Operand::Direct(iter_next_u16(iter)?),
          dest: Operand::Register(RegisterWordType::Ax),
        },
        2 => Op::BinaryByte {
          op: OpBinaryOp::Mov,
          src: Operand::Register(RegisterByteType::Al),
          dest: Operand::Direct(iter_next_u16(iter)?),
        },
        3 => Op::BinaryWord {
          op: OpBinaryOp::Mov,
          src: Operand::Register(RegisterWordType::Ax),
          dest: Operand::Direct(iter_next_u16(iter)?),
        },
        4 => Op::Movs(OpSize::Byte),
        5 => Op::Movs(OpSize::Word),
        6 => Op::Movs(OpSize::Byte),
        7 => Op::Movs(OpSize::Word),
        _ => return None,
      }
    },
    0xA8 => {
      // A8..A9 - TEST
      // AA..AB - STOS
      // AC..AD - LODS
      // AE..AF - SCAS
      match first_octet {
        0 => Op::BinaryByte {
          op: OpBinaryOp::Test,
          src: Operand::ImmByte(iter.next()?),
          dest: Operand::Register(RegisterByteType::Al),
        },
        1 => Op::BinaryWord {
          op: OpBinaryOp::Test,
          src: Operand::ImmWord(iter_next_u16(iter)?),
          dest: Operand::Register(RegisterWordType::Ax),
        },
        2 => Op::Stos(OpSize::Byte),
        3 => Op::Stos(OpSize::Word),
        4 => Op::Lods(OpSize::Byte),
        5 => Op::Lods(OpSize::Word),
        6 => Op::Scas(OpSize::Byte),
        7 => Op::Scas(OpSize::Word),
        _ => return None,
      }
    },
    0xB0 => {
      // MOV
      Op::BinaryByte {
        op: OpBinaryOp::Mov,
        src: Operand::ImmByte(iter.next()?),
        dest: parse_reg(first_octet)?,
      }
    },
    0xB8 => {
      // MOV
      Op::BinaryWord {
        op: OpBinaryOp::Mov,
        src: Operand::ImmWord(iter_next_u16(iter)?),
        dest: parse_reg(first_octet)?,
      }
    },
    0xC0 => {
      // C0 - 
      // C1 - 
      // C2..C3 - RET
      // C4 - LES
      // C5 - LDS
      // C6 - MOV
      // C7 - MOV
      match first_octet {
        0 => return None,
        1 => return None,
        2 => Op::RetWithinImm(iter_next_u16(iter)?),
        3 => Op::RetWithin,
        4 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(second, iter)?;
          let reg = RegisterWordType::from_value((second >> 3) & 0x07)?;
          Op::Les(reg, mod_rm)
        }
        5 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(second, iter)?;
          let reg = RegisterWordType::from_value((second >> 3) & 0x07)?;
          Op::Lds(reg, mod_rm)
        }
        6 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(second, iter)?;
          match (second >> 3) & 0x07 {
            0 => Op::BinaryByte {
              op: OpBinaryOp::Mov,
              src: Operand::ImmByte(iter.next()?),
              dest: mod_rm,
            },
            _ => return None,
          }
        },
        7 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(second, iter)?;
          match (second >> 3) & 0x07 {
            0 => Op::BinaryWord {
              op: OpBinaryOp::Mov,
              src: Operand::ImmWord(iter_next_u16(iter)?),
              dest: mod_rm,
            },
            _ => return None,
          }
        },
        _ => return None,
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
      match first_octet {
        0 => return None,
        1 => return None,
        2 => Op::RetInterImm(iter_next_u16(iter)?),
        3 => Op::RetInter,
        4 => Op::Int(3),
        5 => Op::Int(iter.next()?),
        6 => Op::Nullary(OpNullaryOp::Into),
        7 => Op::Nullary(OpNullaryOp::Iret),
        _ => return None,
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
      match first_octet {
        0 | 2 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(second, iter)?;
          let shift_type = match (first >> 1) & 0x01 {
            0 => OpShiftType::One,
            1 => OpShiftType::Cl,
            _ => return None,
          };
          let shift_op = OpShiftOp::from_value((second >> 3) & 0x07)?;
          Op::ShiftByte {
            op: shift_op,
            shift_type: shift_type,
            dest: mod_rm,
          }
        }
        1 | 3 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(second, iter)?;
          let shift_type = match (first >> 1) & 0x01 {
            0 => OpShiftType::One,
            1 => OpShiftType::Cl,
            _ => return None,
          };
          let shift_op = OpShiftOp::from_value((second >> 3) & 0x07)?;
          Op::ShiftWord {
            op: shift_op,
            shift_type: shift_type,
            dest: mod_rm,
          }
        }
        4 => {
          let second = iter.next()?;
          if second == 0x0A {
            Op::Nullary(OpNullaryOp::Aam)
          } else {
            return None;
          }
        }
        5 => {
          let second = iter.next()?;
          if second == 0x0A {
            Op::Nullary(OpNullaryOp::Aad)
          } else {
            return None;
          }
        }
        6 => return None,
        7 => Op::Nullary(OpNullaryOp::Xlat),
        _ => return None,
      }
    },
    0xD8 => {
      // ESC
      let second = iter.next()?;
      let rm = parse_mod_rm(second, iter)?;
      let esc_id = ((first & 0x7) << 3) + ((second >> 3) & 0x7);
      Op::Esc(esc_id, rm)
    },
    0xE0 => {
      // E0 - LOOPNE
      // E1 - LOOPE
      // E2 - LOOP
      // E3 - JCXZ
      // E4..E5 - IN
      // E6..E7 - OUT
      match first_octet {
        0..=3 => {
          let second = iter.next()?;
          let jmp_type = match first_octet {
            0 => OpCondJmpOp::Loopne,
            1 => OpCondJmpOp::Loope,
            2 => OpCondJmpOp::Loop,
            3 => OpCondJmpOp::Jcxz,
            _ => return None,
          };
          Op::CondJmp { op: jmp_type, offset: second as i8 }
        },
        4 => Op::InVariable(OpSize::Byte, iter.next()?),
        5 => Op::InVariable(OpSize::Word, iter.next()?),
        6 => Op::OutVariable(OpSize::Byte, iter.next()?),
        7 => Op::OutVariable(OpSize::Word, iter.next()?),
        _ => return None,
      }
    },
    0xE8 => {
      // E8 - CALL
      // E9 - JMP
      // EA - JMP
      // EB - JMP
      // EC..ED - IN
      // EE..EF - OUT
      match first_octet {
        0 => Op::Call(OpCallType::WithinDirect(iter_next_u16(iter)?)),
        1 => Op::Jmp(OpCallType::WithinDirect(iter_next_u16(iter)?)),
        2 => Op::Jmp(OpCallType::InterDirect(
          iter_next_u16(iter)?, iter_next_u16(iter)?)),
        3 => Op::Jmp(OpCallType::WithinDirect(iter.next()? as u16)),
        4 => Op::InFixed(OpSize::Byte),
        5 => Op::InFixed(OpSize::Word),
        6 => Op::OutFixed(OpSize::Byte),
        7 => Op::OutFixed(OpSize::Word),
        _ => return None,
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
      match first_octet {
        0 => Op::Nullary(OpNullaryOp::Lock),
        1 => return None,
        2 => Op::Nullary(OpNullaryOp::Repz),
        3 => Op::Nullary(OpNullaryOp::Rep),
        4 => Op::Nullary(OpNullaryOp::Hlt),
        5 => Op::Nullary(OpNullaryOp::Cmc),
        6 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(second, iter)?;
          let type_octet = (second >> 3) & 0x07;
          match type_octet {
            0 => {
              Op::BinaryByte {
                op: OpBinaryOp::Test,
                src: OperandByte::ImmByte(iter.next()?),
                dest: mod_rm,
              }
            },
            _ => {
              let op = OpUnaryOp::from_grp1(type_octet)?;
              Op::UnaryByte {
                op: op,
                dest: mod_rm,
              }
            },
          }
        },
        7 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(second, iter)?;
          let type_octet = (second >> 3) & 0x07;
          match type_octet {
            0 => {
              Op::BinaryWord {
                op: OpBinaryOp::Test,
                src: OperandWord::ImmWord(iter_next_u16(iter)?),
                dest: mod_rm,
              }
            },
            _ => {
              let op = OpUnaryOp::from_grp1(type_octet)?;
              Op::UnaryWord {
                op: op,
                dest: mod_rm,
              }
            },
          }
        },
        _ => return None,
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
      // INC, DEC, CALL, CALL, JMP, JMP, Push, -
      // FF - op MEM16
      match first_octet {
        0 => Op::Nullary(OpNullaryOp::Clc),
        1 => Op::Nullary(OpNullaryOp::Stc),
        2 => Op::Nullary(OpNullaryOp::Cli),
        3 => Op::Nullary(OpNullaryOp::Sti),
        4 => Op::Nullary(OpNullaryOp::Cld),
        5 => Op::Nullary(OpNullaryOp::Std),
        6 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(second, iter)?;
          let type_octet = (second >> 3) & 0x07;
          let op = OpUnaryOp::from_grp2(type_octet)?;
          Op::UnaryByte {
            op: op,
            dest: mod_rm,
          }
        },
        7 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(second, iter)?;
          let type_octet = (second >> 3) & 0x07;
          match type_octet {
            2 => Op::Call(OpCallType::WithinIndirect(mod_rm)),
            3 => Op::Call(OpCallType::InterIndirect(mod_rm)),
            4 => Op::Jmp(OpCallType::WithinIndirect(mod_rm)),
            5 => Op::Jmp(OpCallType::InterIndirect(mod_rm)),
            _ => {
              let op = OpUnaryOp::from_grp2(type_octet)?;
              Op::UnaryWord {
                op: op,
                dest: mod_rm,
              }
            },
          }
        },
        _ => return None,
      }
    },
    _ => return None,
  })
}

#[test]
fn test_parse_add() {
  {
    let input: Vec<u8> = vec![0x00, 0xC0];
    assert_eq!(
      parse_op(&mut input.into_iter()),
      Some(Op::BinaryByte {
        op: OpBinaryOp::Add,
        src: Operand::Register(RegisterByteType::Al),
        dest: Operand::Register(RegisterByteType::Al),
      }),
    );
  }
  {
    let input: Vec<u8> = vec![0x8b, 0xE2];
    assert_eq!(
      parse_op(&mut input.into_iter()),
      Some(Op::BinaryWord {
        op: OpBinaryOp::Mov,
        src: Operand::Register(RegisterWordType::Dx),
        dest: Operand::Register(RegisterWordType::Sp),
      }),
    );
  }
  {
    let input: Vec<u8> = vec![0x89, 0xE2];
    assert_eq!(
      parse_op(&mut input.into_iter()),
      Some(Op::BinaryWord {
        op: OpBinaryOp::Mov,
        src: Operand::Register(RegisterWordType::Sp),
        dest: Operand::Register(RegisterWordType::Dx),
      }),
    );
  }
  {
    let input: Vec<u8> = vec![0x89, 0xc3];
    assert_eq!(
      parse_op(&mut input.into_iter()),
      Some(Op::BinaryWord {
        op: OpBinaryOp::Mov,
        src: Operand::Register(RegisterWordType::Ax),
        dest: Operand::Register(RegisterWordType::Bx),
      }),
    );
  }
  {
    let input: Vec<u8> = vec![0x88, 0xc3];
    assert_eq!(
      parse_op(&mut input.into_iter()),
      Some(Op::BinaryByte {
        op: OpBinaryOp::Mov,
        src: Operand::Register(RegisterByteType::Al),
        dest: Operand::Register(RegisterByteType::Bl),
      }),
    );
  }
  {
    let input: Vec<u8> = vec![0x06];
    assert_eq!(
      parse_op(&mut input.into_iter()),
      Some(Op::UnaryWord {
        op: OpUnaryOp::Push,
        dest: Operand::Register(RegisterWordType::Es),
      }),
    );
  }
  {
    let input: Vec<u8> = vec![0x01, 0xd4];
    assert_eq!(
      parse_op(&mut input.into_iter()),
      Some(Op::BinaryWord {
        op: OpBinaryOp::Add,
        src: Operand::Register(RegisterWordType::Dx),
        dest: Operand::Register(RegisterWordType::Sp),
      }),
    );
  }
  {
    let input: Vec<u8> = vec![0x31, 0x80, 0xab, 0xcd];
    assert_eq!(
      parse_op(&mut input.into_iter()),
      Some(Op::BinaryWord {
        op: OpBinaryOp::Xor,
        src: Operand::Register(RegisterWordType::Ax),
        dest: Operand::Address(AddressType::BxSi, 0xcdab as u16 as i16),
      }),
    );
  }
  {
    let input: Vec<u8> = vec![0x80, 0x80, 0xab, 0xcd, 0x25];
    assert_eq!(
      parse_op(&mut input.into_iter()),
      Some(Op::BinaryByte {
        op: OpBinaryOp::Add,
        src: Operand::ImmByte(0x25),
        dest: Operand::Address(AddressType::BxSi, 0xcdab as u16 as i16),
      }),
    );
  }
  {
    let input: Vec<u8> = vec![0x81, 0xc3, 0x00, 0xf0];
    assert_eq!(
      parse_op(&mut input.into_iter()),
      Some(Op::BinaryWord {
        op: OpBinaryOp::Add,
        src: Operand::ImmWord(0xf000),
        dest: Operand::Register(RegisterWordType::Bx),
      }),
    );
  }
}
