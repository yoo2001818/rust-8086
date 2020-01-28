
#[derive(PartialEq, Copy, Clone)]
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

impl OpRegister {
  fn from_value(size: OpSize, value: u8) -> Option<OpRegister> {
    Some(match size {
      OpSize::Byte => match value {
        0 => OpRegister::Al,
        1 => OpRegister::Cl,
        2 => OpRegister::Dl,
        3 => OpRegister::Bl,
        4 => OpRegister::Ah,
        5 => OpRegister::Ch,
        6 => OpRegister::Dh,
        7 => OpRegister::Bh,
        _ => return None,
      },
      OpSize::Word => match value {
        0 => OpRegister::Ax,
        1 => OpRegister::Cx,
        2 => OpRegister::Dx,
        3 => OpRegister::Bx,
        4 => OpRegister::Sp,
        5 => OpRegister::Bp,
        6 => OpRegister::Si,
        7 => OpRegister::Di,
        _ => return None,
      },
    })
  }
  fn to_value(&self) -> u8 {
    (*self as u8) & 0x7
  }
}

#[derive(PartialEq, Copy, Clone)]
#[derive(Debug)]
pub enum OpSegmentRegister {
  Es,
  Cs,
  Ss,
  Ds,
}

impl OpSegmentRegister {
  fn from_value(value: u8) -> Option<OpSegmentRegister> {
    Some(match value {
      0 => OpSegmentRegister::Es,
      1 => OpSegmentRegister::Cs,
      2 => OpSegmentRegister::Ss,
      3 => OpSegmentRegister::Ds,
      _ => return None,
    })
  }
  fn to_value(&self) -> u8 {
    *self as u8
  }
}

#[derive(PartialEq, Copy, Clone)]
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

impl OpAddressType {
  fn from_value(value: u8) -> Option<OpAddressType> {
    Some(match value {
      0 => OpAddressType::BxSi,
      1 => OpAddressType::BxDi,
      2 => OpAddressType::BpSi,
      3 => OpAddressType::BpDi,
      4 => OpAddressType::Si,
      5 => OpAddressType::Di,
      6 => OpAddressType::Bp,
      7 => OpAddressType::Bx,
      _ => return None,
    })
  }
  fn to_value(&self) -> u8 {
    *self as u8
  }
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
  WithinIndirect(OpTarget),
  InterDirect(u16, u16),
  InterIndirect(OpTarget),
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

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Op {
  Binary { op: OpBinaryOp, size: OpSize, src: OpTarget, dest: OpTarget },
  Unary { op: OpUnaryOp, size: OpSize, dest: OpTarget },
  Shift {
    op: OpShiftOp,
    shift_type: OpShiftType,
    size: OpSize,
    dest: OpTarget,
  },
  CondJmp { op: OpCondJmpOp, offset: u8 },
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
  Movs(OpSize),
  Cmps(OpSize),
  Scas(OpSize),
  Lods(OpSize),
  Stos(OpSize),
  Rep,
  Repz,
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
  Esc(u8, OpTarget),
  Lock,
  Segment(OpSegmentRegister),
}

fn iter_next_u16(iter: &mut dyn Iterator<Item = u8>) -> Option<u16> {
  Some(iter.next()? as u16 +
    ((iter.next()? as u16) << 8))
}

pub fn parse_mod_rm(
  size: OpSize,
  second: u8,
  iter: &mut dyn Iterator<Item = u8>,
) -> Option<OpTarget> {
  let mod_val = (second >> 6) & 0x03;
  let rm_val = second & 0x07;
  Some(match mod_val {
    0 => {
      let addr_type = OpAddressType::from_value(rm_val)?;
      if addr_type == OpAddressType::Bp {
        OpTarget::Direct(iter_next_u16(iter)?)
      } else {
        OpTarget::Address(addr_type, 0)   
      }
    },
    1 => OpTarget::Address(
      OpAddressType::from_value(rm_val)?, iter.next()? as u16),
    2 => OpTarget::Address(
      OpAddressType::from_value(rm_val)?, iter_next_u16(iter)?),
    3 => OpTarget::Register(OpRegister::from_value(size, rm_val)?),
    _ => return None,
  })
}

pub fn parse_binary_group_op(
  op: OpBinaryOp,
  first: u8,
  iter: &mut dyn Iterator<Item = u8>,
) -> Option<Op> {
  Some(match first & 0x07 {
    0..=3 => {
      let second = iter.next()?;
      let size = match first & 0x01 {
        0 => OpSize::Byte,
        1 => OpSize::Word,
        _ => panic!("This should never happen"),
      };
      let mod_rm = parse_mod_rm(size, second, iter)?;
      let reg = OpTarget::Register(
        OpRegister::from_value(size, (second >> 3) & 0x07)?);
      match (first >> 1) & 0x01 {
        0 => Op::Binary {
          op: op,
          size: size,
          src: reg,
          dest: mod_rm,
        },
        1 => Op::Binary {
          op: op,
          size: size,
          src: mod_rm,
          dest: reg,
        },
        _ => panic!("This should never happen"),
      }
    }
    4 => Op::Binary {
      op: op,
      size: OpSize::Byte,
      src: OpTarget::ImmByte(iter.next()?),
      dest: OpTarget::Register(OpRegister::Al),
    },
    5 => Op::Binary {
      op: op,
      size: OpSize::Word,
      src: OpTarget::ImmWord(iter_next_u16(iter)?),
      dest: OpTarget::Register(OpRegister::Ax),
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
        6 => Op::Unary {
          op: OpUnaryOp::Push,
          size: OpSize::Word,
          dest: OpTarget::SegmentRegister(OpSegmentRegister::Es),
        },
        7 => Op::Unary {
          op: OpUnaryOp::Pop,
          size: OpSize::Word,
          dest: OpTarget::SegmentRegister(OpSegmentRegister::Es),
        },
        _ => return None,
      }
    },
    0x08 => {
      match first_octet {
        0..=5 => parse_binary_group_op(OpBinaryOp::Or, first, iter)?,
        6 => Op::Unary {
          op: OpUnaryOp::Push,
          size: OpSize::Word,
          dest: OpTarget::SegmentRegister(OpSegmentRegister::Cs),
        },
        7 => Op::Unary {
          op: OpUnaryOp::Pop,
          size: OpSize::Word,
          dest: OpTarget::SegmentRegister(OpSegmentRegister::Cs),
        },
        _ => return None,
      }
    },
    0x10 => {
      match first_octet {
        0..=5 => parse_binary_group_op(OpBinaryOp::Adc, first, iter)?,
        6 => Op::Unary {
          op: OpUnaryOp::Push,
          size: OpSize::Word,
          dest: OpTarget::SegmentRegister(OpSegmentRegister::Ss),
        },
        7 => Op::Unary {
          op: OpUnaryOp::Pop,
          size: OpSize::Word,
          dest: OpTarget::SegmentRegister(OpSegmentRegister::Ss),
        },
        _ => return None,
      }
    },
    0x18 => {
      match first_octet {
        0..=5 => parse_binary_group_op(OpBinaryOp::Sbb, first, iter)?,
        6 => Op::Unary {
          op: OpUnaryOp::Push,
          size: OpSize::Word,
          dest: OpTarget::SegmentRegister(OpSegmentRegister::Ds),
        },
        7 => Op::Unary {
          op: OpUnaryOp::Pop,
          size: OpSize::Word,
          dest: OpTarget::SegmentRegister(OpSegmentRegister::Ds),
        },
        _ => return None,
      }
    },
    0x20 => {
      match first_octet {
        0..=5 => parse_binary_group_op(OpBinaryOp::And, first, iter)?,
        6 => Op::Segment(OpSegmentRegister::Es),
        7 => Op::Daa,
        _ => return None,
      }
    },
    0x28 => {
      match first_octet {
        0..=5 => parse_binary_group_op(OpBinaryOp::Sub, first, iter)?,
        6 => Op::Segment(OpSegmentRegister::Cs),
        7 => Op::Das,
        _ => return None,
      }
    },
    0x30 => {
      match first_octet {
        0..=5 => parse_binary_group_op(OpBinaryOp::Xor, first, iter)?,
        6 => Op::Segment(OpSegmentRegister::Ss),
        7 => Op::Aaa,
        _ => return None,
      }
    },
    0x38 => {
      match first_octet {
        0..=5 => parse_binary_group_op(OpBinaryOp::Cmp, first, iter)?,
        6 => Op::Segment(OpSegmentRegister::Ds),
        7 => Op::Aas,
        _ => return None,
      }
    },
    0x40 => {
      Op::Unary {
        op: OpUnaryOp::Inc,
        size: OpSize::Word,
        dest: OpTarget::Register(
          OpRegister::from_value(OpSize::Word, first_octet)?),
      }
    },
    0x48 => {
      Op::Unary {
        op: OpUnaryOp::Dec,
        size: OpSize::Word,
        dest: OpTarget::Register(
          OpRegister::from_value(OpSize::Word, first_octet)?),
      }
    },
    0x50 => {
      Op::Unary {
        op: OpUnaryOp::Push,
        size: OpSize::Word,
        dest: OpTarget::Register(
          OpRegister::from_value(OpSize::Word, first_octet)?),
      }
    },
    0x58 => {
      Op::Unary {
        op: OpUnaryOp::Pop,
        size: OpSize::Word,
        dest: OpTarget::Register(
          OpRegister::from_value(OpSize::Word, first_octet)?),
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
        _ => panic!("This should not happen"),
      };
      Op::CondJmp { op: jmp_type, offset: second }
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
        _ => panic!("This should not happen"),
      };
      Op::CondJmp { op: jmp_type, offset: second }
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
          let size = match first & 0x01 {
            0 => OpSize::Byte,
            1 => OpSize::Word,
            _ => panic!("This should never happen"),
          };
          let mod_rm = parse_mod_rm(size, second, iter)?;
          let op = OpBinaryOp::from_immed((second >> 3) & 0x07)?;
          Op::Binary {
            op: op,
            size: size,
            src: match first & 0x03 {
              1 => OpTarget::ImmWord(iter_next_u16(iter)?),
              _ => OpTarget::ImmByte(iter.next()?),
            },
            dest: mod_rm,
          }
        },
        4..=5 => {
          let second = iter.next()?;
          let size = match first & 0x01 {
            0 => OpSize::Byte,
            1 => OpSize::Word,
            _ => panic!("This should never happen"),
          };
          let mod_rm = parse_mod_rm(size, second, iter)?;
          let reg = OpRegister::from_value(size, (second >> 3) & 0x07)?;
          Op::Binary {
            op: OpBinaryOp::Test,
            size: size,
            src: OpTarget::Register(reg),
            dest: mod_rm,
          }
        },
        6..=7 => {
          let second = iter.next()?;
          let size = match first & 0x01 {
            0 => OpSize::Byte,
            1 => OpSize::Word,
            _ => panic!("This should never happen"),
          };
          let mod_rm = parse_mod_rm(size, second, iter)?;
          let reg = OpRegister::from_value(size, (second >> 3) & 0x07)?;
          Op::Binary {
            op: OpBinaryOp::Xchg,
            size: size,
            src: OpTarget::Register(reg),
            dest: mod_rm,
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
          let mod_rm = parse_mod_rm(OpSize::Word, second, iter)?;
          let reg = OpSegmentRegister::from_value((second >> 3) & 0x07)?;
          Op::Binary {
            op: OpBinaryOp::Mov,
            size: OpSize::Word,
            src: OpTarget::SegmentRegister(reg),
            dest: mod_rm,
          }
        },
        5 => {
          // LEA reg16, r/m16
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(OpSize::Word, second, iter)?;
          let reg = OpRegister::from_value(OpSize::Word, (second >> 3) & 0x07)?;
          Op::Lea(reg, mod_rm)
        },
        6 => {
          // MOV segreg, r/m16
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(OpSize::Word, second, iter)?;
          let reg = OpSegmentRegister::from_value((second >> 3) & 0x07)?;
          Op::Binary {
            op: OpBinaryOp::Mov,
            size: OpSize::Word,
            src: mod_rm,
            dest: OpTarget::SegmentRegister(reg),
          }
        },
        7 => {
          // Pop r/m16 (second 000)
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(OpSize::Word, second, iter)?;
          match (second >> 3) & 0x07 {
            0 => Op::Unary {
              op: OpUnaryOp::Pop,
              size: OpSize::Word,
              dest: mod_rm,
            },
            _ => return None,
          }
        },
        _ => return None,
      }
    },
    0x90 => {
      Op::Binary {
        op: OpBinaryOp::Xchg,
        size: OpSize::Word,
        src: OpTarget::Register(OpRegister::Ax),
        dest: OpTarget::Register(
          OpRegister::from_value(OpSize::Word, first_octet)?),
      }
    },
    0x98 => {
      match first_octet {
        0 => Op::Cbw,
        1 => Op::Cwd,
        2 => Op::Call(OpCallType::InterDirect(iter_next_u16(iter)?, iter_next_u16(iter)?)),
        3 => Op::Wait,
        4 => Op::Pushf,
        5 => Op::Popf,
        6 => Op::Sahf,
        7 => Op::Lahf,
        _ => return None,
      }
    },
    0xA0 => {
      // A0..A3 - MOV
      // A4..A5 - MOVS
      // A6..A7 - CMPS
      match first_octet {
        0 => Op::Binary {
          op: OpBinaryOp::Mov,
          size: OpSize::Byte,
          src: OpTarget::Direct(iter_next_u16(iter)?),
          dest: OpTarget::Register(OpRegister::Al),
        },
        1 => Op::Binary {
          op: OpBinaryOp::Mov,
          size: OpSize::Word,
          src: OpTarget::Direct(iter_next_u16(iter)?),
          dest: OpTarget::Register(OpRegister::Ax),
        },
        2 => Op::Binary {
          op: OpBinaryOp::Mov,
          size: OpSize::Byte,
          src: OpTarget::Register(OpRegister::Al),
          dest: OpTarget::Direct(iter_next_u16(iter)?),
        },
        3 => Op::Binary {
          op: OpBinaryOp::Mov,
          size: OpSize::Word,
          src: OpTarget::Register(OpRegister::Ax),
          dest: OpTarget::Direct(iter_next_u16(iter)?),
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
        0 => Op::Binary {
          op: OpBinaryOp::Test,
          size: OpSize::Byte,
          src: OpTarget::ImmByte(iter.next()?),
          dest: OpTarget::Register(OpRegister::Al),
        },
        1 => Op::Binary {
          op: OpBinaryOp::Test,
          size: OpSize::Word,
          src: OpTarget::ImmWord(iter_next_u16(iter)?),
          dest: OpTarget::Register(OpRegister::Ax),
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
      Op::Binary {
        op: OpBinaryOp::Mov,
        size: OpSize::Byte,
        src: OpTarget::ImmByte(iter.next()?),
        dest: OpTarget::Register(
          OpRegister::from_value(OpSize::Byte, first_octet)?),
      }
    },
    0xB8 => {
      // MOV
      Op::Binary {
        op: OpBinaryOp::Mov,
        size: OpSize::Word,
        src: OpTarget::ImmWord(iter_next_u16(iter)?),
        dest: OpTarget::Register(
          OpRegister::from_value(OpSize::Word, first_octet)?),
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
          let mod_rm = parse_mod_rm(OpSize::Word, second, iter)?;
          let reg = OpRegister::from_value(OpSize::Word, (second >> 3) & 0x07)?;
          Op::Les(reg, mod_rm)
        }
        5 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(OpSize::Word, second, iter)?;
          let reg = OpRegister::from_value(OpSize::Word, (second >> 3) & 0x07)?;
          Op::Lds(reg, mod_rm)
        }
        6 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(OpSize::Byte, second, iter)?;
          match (second >> 3) & 0x07 {
            0 => Op::Binary {
              op: OpBinaryOp::Mov,
              size: OpSize::Byte,
              src: OpTarget::ImmByte(iter.next()?),
              dest: mod_rm,
            },
            _ => return None,
          }
        },
        7 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(OpSize::Word, second, iter)?;
          match (second >> 3) & 0x07 {
            0 => Op::Binary {
              op: OpBinaryOp::Mov,
              size: OpSize::Word,
              src: OpTarget::ImmWord(iter_next_u16(iter)?),
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
        6 => Op::Into,
        7 => Op::Iret,
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
        0..=3 => {
          let second = iter.next()?;
          let size = match first & 0x01 {
            0 => OpSize::Byte,
            1 => OpSize::Word,
            _ => panic!("This should never happen"),
          };
          let mod_rm = parse_mod_rm(size, second, iter)?;
          let shift_type = match (first >> 1) & 0x01 {
            0 => OpShiftType::One,
            1 => OpShiftType::Cl,
            _ => return None,
          };
          let shift_op = OpShiftOp::from_value((second >> 3) & 0x07)?;
          Op::Shift {
            op: shift_op,
            shift_type: shift_type,
            size: size,
            dest: mod_rm,
          }
        },
        4 => {
          let second = iter.next()?;
          if second == 0x0A {
            Op::Aam
          } else {
            return None;
          }
        }
        5 => {
          let second = iter.next()?;
          if second == 0x0A {
            Op::Aad
          } else {
            return None;
          }
        }
        6 => return None,
        7 => Op::Xlat,
        _ => return None,
      }
    },
    0xD8 => {
      // ESC
      let second = iter.next()?;
      let rm = parse_mod_rm(OpSize::Word, second, iter)?;
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
            _ => panic!("This should not happen"),
          };
          Op::CondJmp { op: jmp_type, offset: second }
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
        0 => Op::Lock,
        1 => return None,
        2 => Op::Repz,
        3 => Op::Rep,
        4 => Op::Hlt,
        5 => Op::Cmc,
        6..=7 => {
          let second = iter.next()?;
          let size = match first & 0x01 {
            0 => OpSize::Byte,
            1 => OpSize::Word,
            _ => panic!("This should never happen"),
          };
          let mod_rm = parse_mod_rm(size, second, iter)?;
          let type_octet = (second >> 3) & 0x07;
          match type_octet {
            0 => {
              let imm = match size {
                OpSize::Byte => OpTarget::ImmByte(iter.next()?),
                OpSize::Word => OpTarget::ImmWord(iter_next_u16(iter)?),
              };
              Op::Binary {
                op: OpBinaryOp::Test,
                size: size,
                src: imm,
                dest: mod_rm,
              }
            },
            _ => {
              let op = OpUnaryOp::from_grp1(type_octet)?;
              Op::Unary {
                op: op,
                size: size,
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
        0 => Op::Clc,
        1 => Op::Stc,
        2 => Op::Cli,
        3 => Op::Sti,
        4 => Op::Cld,
        5 => Op::Std,
        6 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(OpSize::Byte, second, iter)?;
          let type_octet = (second >> 3) & 0x07;
          let op = OpUnaryOp::from_grp2(type_octet)?;
          Op::Unary {
            op: op,
            size: OpSize::Byte,
            dest: mod_rm,
          }
        },
        7 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm(OpSize::Word, second, iter)?;
          let type_octet = (second >> 3) & 0x07;
          match type_octet {
            2 => Op::Call(OpCallType::WithinIndirect(mod_rm)),
            3 => Op::Call(OpCallType::InterIndirect(mod_rm)),
            4 => Op::Jmp(OpCallType::WithinIndirect(mod_rm)),
            5 => Op::Jmp(OpCallType::InterIndirect(mod_rm)),
            _ => {
              let op = OpUnaryOp::from_grp2(type_octet)?;
              Op::Unary {
                op: op,
                size: OpSize::Word,
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
      Some(Op::Binary {
        op: OpBinaryOp::Add,
        size: OpSize::Byte,
        src: OpTarget::Register(OpRegister::Al),
        dest: OpTarget::Register(OpRegister::Al),
      }),
    );
  }
  {
    let input: Vec<u8> = vec![0x06];
    assert_eq!(
      parse_op(&mut input.into_iter()),
      Some(Op::Unary {
        op: OpUnaryOp::Push,
        size: OpSize::Word,
        dest: OpTarget::SegmentRegister(OpSegmentRegister::Es),
      }),
    );
  }
  {
    let input: Vec<u8> = vec![0x31, 0x80, 0xab, 0xcd];
    println!("{:#?}", parse_op(&mut input.into_iter()));
  }
  {
    let input: Vec<u8> = vec![0x80, 0x80, 0xab, 0xcd, 0x25];
    println!("{:#?}", parse_op(&mut input.into_iter()));
  }
  {
    let input: Vec<u8> = vec![0x81, 0xc3, 0x00, 0xf0];
    println!("{:#?}", parse_op(&mut input.into_iter()));
  }
}
