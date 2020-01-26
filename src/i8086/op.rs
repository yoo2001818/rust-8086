use std::iter::Iterator;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpRegisterWord {
  Ax,
  Cx,
  Dx,
  Bx,
  Sp,
  Bp,
  Si,
  Di,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpRegisterByte {
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
pub enum OpModRm<T> {
  Register(T),
  Address(OpAddressType),
  AddressDispByte(OpAddressType, u8),
  AddressDispWord(OpAddressType, u16),
  Direct(u16),
}

pub type OpModRmWord = OpModRm<OpRegisterWord>;
pub type OpModRmByte = OpModRm<OpRegisterByte>;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpModRmDual {
  Word(OpModRmWord),
  Byte(OpModRmByte),
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpDirectionType {
  RegToRm,
  RmToReg,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct OpModRegRm<T>(T, OpModRm<T>);

pub type OpModRegRmWord = OpModRegRm<OpRegisterWord>;
pub type OpModRegRmByte = OpModRegRm<OpRegisterByte>;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpModRegRmDual {
  Word(OpModRegRmWord),
  Byte(OpModRegRmByte),
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpBinarySrcDest<T, V> {
  RegToRm(OpModRegRm<T>),
  RmToReg(OpModRegRm<T>),
  ImmRm(OpModRm<T>, V),
  ImmReg(T, V),
  ImmAx(V),
}

pub type OpBinarySrcDestWord = OpBinarySrcDest<OpRegisterWord, u16>;
pub type OpBinarySrcDestByte = OpBinarySrcDest<OpRegisterByte, u8>;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpBinarySrcDestDual {
  Word(OpBinarySrcDestWord),
  Byte(OpBinarySrcDestByte),
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpRotateType {
  One,
  Cl,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum OpWordByte {
  Byte,
  Word,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Op {
  Mov(OpBinarySrcDestDual),
  MovWordSeg(OpDirectionType, OpModRmWord, OpSegmentRegister),
  MovAxToMem(u16),
  MovMemToAx(u16),
  MovAlToMem(u16),
  MovMemToAl(u16),
  PushRm(OpModRmWord),
  PushReg(OpRegisterWord),
  PushSeg(OpSegmentRegister),
  PopRm(OpModRmWord),
  PopReg(OpRegisterWord),
  PopSeg(OpSegmentRegister),
  XchgRmReg(OpModRegRmDual),
  XchgRegAx(OpRegisterWord),
  InFixed(OpWordByte),
  InVariable(OpWordByte, u8),
  OutFixed(OpWordByte),
  OutVariable(OpWordByte, u8),
  Xlat,
  Lea(OpModRegRmWord),
  Lds(OpModRegRmWord),
  Les(OpModRegRmWord),
  Lahf,
  Sahf,
  Pushf,
  Popf,
  Add(OpBinarySrcDestDual),
  Adc(OpBinarySrcDestDual),
  IncRm(OpModRmDual),
  IncReg(OpRegisterWord),
  Aaa,
  Daa,
  Sub(OpBinarySrcDestDual),
  Sbb(OpBinarySrcDestDual),
  DecRm(OpModRmDual),
  DecReg(OpRegisterWord),
  Neg(OpModRmDual),
  Cmp(OpBinarySrcDestDual),
  Aas,
  Das,
  Mul(OpModRmDual),
  Imul(OpModRmDual),
  Aam,
  Div(OpModRmDual),
  Idiv(OpModRmDual),
  Aad,
  Cbw,
  Cwd,
  Not(OpModRmDual),
  Shl(OpRotateType, OpModRmDual),
  Shr(OpRotateType, OpModRmDual),
  Sar(OpRotateType, OpModRmDual),
  Rol(OpRotateType, OpModRmDual),
  Ror(OpRotateType, OpModRmDual),
  Rcl(OpRotateType, OpModRmDual),
  Rcr(OpRotateType, OpModRmDual),
  And(OpBinarySrcDestDual),
  Test(OpModRegRmDual),
  TestImmByte(OpModRmByte, u8),
  TestImmWord(OpModRmWord, u16),
  TestImmAl(u8),
  TestImmAx(u16),
  Or(OpBinarySrcDestDual),
  Xor(OpBinarySrcDestDual),
  RepSet,
  RepUnset,
  Movs(OpWordByte),
  Cmps(OpWordByte),
  Scas(OpWordByte),
  Lods(OpWordByte),
  Stos(OpWordByte),
  CallWithinDirect(u16),
  CallWithinIndirect(OpModRmWord),
  CallInterDirect(u16, u16),
  CallInterIndirect(OpModRmWord),
  JmpWithinDirect(u16),
  JmpWithinDirectShort(u8),
  JmpWithinIndirect(OpModRmWord),
  JmpInterDirect(u16, u16),
  JmpInterIndirect(OpModRmWord),
  RetWithin,
  RetWithinImm(u16),
  RetInter,
  RetInterImm(u16),
  Je(u8),
  Jl(u8),
  Jle(u8),
  Jb(u8),
  Jbe(u8),
  Jp(u8),
  Jo(u8),
  Js(u8),
  Jne(u8),
  Jge(u8),
  Jg(u8),
  Jae(u8),
  Ja(u8),
  Jnp(u8),
  Jno(u8),
  Jns(u8),
  Loop(u8),
  Loopz(u8),
  Loopnz(u8),
  Jcxz(u8),
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
  Esc(u8, OpModRmWord),
  Lock,
  Segment(OpSegmentRegister),
}

pub fn parse_register_word(byte: u8) -> Option<OpRegisterWord> {
  Some(match byte {
    0 => OpRegisterWord::Ax,
    1 => OpRegisterWord::Cx,
    2 => OpRegisterWord::Dx,
    3 => OpRegisterWord::Bx,
    4 => OpRegisterWord::Sp,
    5 => OpRegisterWord::Bp,
    6 => OpRegisterWord::Si,
    7 => OpRegisterWord::Di,
    _ => return None,
  })
}

pub fn parse_register_byte(byte: u8) -> Option<OpRegisterByte> {
  Some(match byte {
    0 => OpRegisterByte::Al,
    1 => OpRegisterByte::Cl,
    2 => OpRegisterByte::Dl,
    3 => OpRegisterByte::Bl,
    4 => OpRegisterByte::Ah,
    5 => OpRegisterByte::Ch,
    6 => OpRegisterByte::Dh,
    7 => OpRegisterByte::Bh,
    _ => return None,
  })
}

pub fn parse_address_type(byte: u8) -> Option<OpAddressType> {
  Some(match byte {
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

pub fn parse_segment_register(byte: u8) -> Option<OpSegmentRegister> {
  Some(match byte {
    0 => OpSegmentRegister::Es,
    1 => OpSegmentRegister::Cs,
    2 => OpSegmentRegister::Ds,
    3 => OpSegmentRegister::Ss,
    _ => return None,
  })
}

fn iter_next_u16 (iter: &mut dyn Iterator<Item = u8>) -> Option<u16> {
  Some(iter.next()? as u16 +
    ((iter.next()? as u16) << 8))
}

pub fn parse_mod_rm_word(
  byte: u8,
  iter: &mut dyn Iterator<Item = u8>,
) -> Option<OpModRmWord> {
  let mod_val = (byte >> 5) & 0x07;
  let rm_val = byte & 0x03;
  Some(match rm_val {
    0 => OpModRmWord::Register(parse_register_word(mod_val)?),
    1 => {
      let addr_type = parse_address_type(mod_val)?;
      if addr_type == OpAddressType::Bp {
        OpModRmWord::Direct(iter_next_u16(iter)?)
      } else {
        OpModRmWord::Address(addr_type)
      }
    }
    2 => OpModRmWord::AddressDispByte(parse_address_type(mod_val)?,
      iter.next()?),
    3 => OpModRmWord::AddressDispWord(parse_address_type(mod_val)?,
      iter_next_u16(iter)?),
    _ => return None,
  })
}

pub fn parse_mod_rm_byte(
  byte: u8,
  iter: &mut dyn Iterator<Item = u8>,
) -> Option<OpModRmByte> {
  let mod_val = (byte >> 5) & 0x07;
  let rm_val = byte & 0x03;
  Some(match rm_val {
    0 => OpModRmByte::Register(parse_register_byte(mod_val)?),
    1 => {
      let addr_type = parse_address_type(mod_val)?;
      if addr_type == OpAddressType::Bp {
        OpModRmByte::Direct(iter_next_u16(iter)?)
      } else {
        OpModRmByte::Address(addr_type)
      }
    }
    2 => OpModRmByte::AddressDispByte(parse_address_type(mod_val)?,
      iter.next()?),
    3 => OpModRmByte::AddressDispWord(parse_address_type(mod_val)?,
      iter_next_u16(iter)?),
    _ => return None,
  })
}

pub fn parse_mod_reg_rm_byte(
  byte: u8, iter: &mut dyn Iterator<Item = u8>,
) -> Option<OpModRegRmByte> {
  let reg = parse_register_byte((byte >> 3) & 0x7)?;
  let rm = parse_mod_rm_byte(byte, iter)?;
  return Some(OpModRegRm::<OpRegisterByte>(reg, rm));
}

pub fn parse_mod_reg_rm_word(
  byte: u8, iter: &mut dyn Iterator<Item = u8>,
) -> Option<OpModRegRmWord> {
  let reg = parse_register_word((byte >> 3) & 0x7)?;
  let rm = parse_mod_rm_word(byte, iter)?;
  return Some(OpModRegRm::<OpRegisterWord>(reg, rm));
}

pub fn parse_binary_src_dest(
  first: u8, iter: &mut dyn Iterator<Item = u8>,
) -> Option<OpBinarySrcDestDual> {
  Some(match first & 0x07 {
    0..=3 => {
      let second = iter.next()?;
      match first & 0x03 {
        0 => OpBinarySrcDestDual::Byte(OpBinarySrcDestByte::RegToRm(parse_mod_reg_rm_byte(second, iter)?)),
        1 => OpBinarySrcDestDual::Word(OpBinarySrcDestWord::RegToRm(parse_mod_reg_rm_word(second, iter)?)),
        2 => OpBinarySrcDestDual::Byte(OpBinarySrcDestByte::RmToReg(parse_mod_reg_rm_byte(second, iter)?)),
        3 => OpBinarySrcDestDual::Word(OpBinarySrcDestWord::RmToReg(parse_mod_reg_rm_word(second, iter)?)),
        _ => panic!("This should never happen"),
      }
    },
    4 => OpBinarySrcDestDual::Byte(
      OpBinarySrcDestByte::ImmAx(iter.next()?)),
    5 => OpBinarySrcDestDual::Word(
      OpBinarySrcDestWord::ImmAx(iter_next_u16(iter)?)),
    _ => panic!("..."),
  })
}

pub fn parse_op(iter: &mut dyn Iterator<Item = u8>) -> Option<Op> {
  let first = iter.next()?;
  let first_octet = first & 0x07;
  Some(match first & 0xf8 {
    0x00 => {
      // ADD, PUSH ES, POP ES
      match first & 0x07 {
        0..=5 => Op::Add(parse_binary_src_dest(first, iter)?),
        6 => Op::PushSeg(OpSegmentRegister::Es),
        7 => Op::PopSeg(OpSegmentRegister::Es),
        _ => return None,
      }
    }
    0x08 => {
      // OR, PUSH CS
      match first_octet {
        0..=5 => Op::Or(parse_binary_src_dest(first, iter)?),
        6 => Op::PushSeg(OpSegmentRegister::Cs),
        7 => Op::PopSeg(OpSegmentRegister::Cs),
        _ => return None,
      }
    },
    0x10 => {
      // ADC, PUSH SS, POP SS
      match first_octet {
        0..=5 => Op::Adc(parse_binary_src_dest(first, iter)?),
        6 => Op::PushSeg(OpSegmentRegister::Ss),
        7 => Op::PopSeg(OpSegmentRegister::Ss),
        _ => return None,
      }
    },
    0x18 => {
      // SBB, PUSH DS, POP DS
      match first_octet {
        0..=5 => Op::Sbb(parse_binary_src_dest(first, iter)?),
        6 => Op::PushSeg(OpSegmentRegister::Ds),
        7 => Op::PopSeg(OpSegmentRegister::Ds),
        _ => return None,
      }
    },
    0x20 => {
      // AND, SELECT ES, DAA
      match first_octet {
        0..=5 => Op::And(parse_binary_src_dest(first, iter)?),
        6 => Op::Segment(OpSegmentRegister::Es),
        7 => Op::Daa,
        _ => return None,
      }
    },
    0x28 => {
      // SUB, SELECT CS, DAS
      match first_octet {
        0..=5 => Op::Sub(parse_binary_src_dest(first, iter)?),
        6 => Op::Segment(OpSegmentRegister::Cs),
        7 => Op::Das,
        _ => return None,
      }
    },
    0x30 => {
      // XOR, SELECT SS, AAA
      match first_octet {
        0..=5 => Op::Xor(parse_binary_src_dest(first, iter)?),
        6 => Op::Segment(OpSegmentRegister::Ss),
        7 => Op::Aaa,
        _ => return None,
      }
    },
    0x38 => {
      // CMP, SELECT DS, AAS
      match first_octet {
        0..=5 => Op::Cmp(parse_binary_src_dest(first, iter)?),
        6 => Op::Segment(OpSegmentRegister::Ds),
        7 => Op::Aas,
        _ => return None,
      }
    },
    0x40 => {
      // INC
      Op::IncReg(parse_register_word(first_octet)?)
    },
    0x48 => {
      // DEC
      Op::DecReg(parse_register_word(first_octet)?)
    },
    0x50 => {
      // PUSH
      Op::PushReg(parse_register_word(first_octet)?)
    },
    0x58 => {
      // POP
      Op::PopReg(parse_register_word(first_octet)?)
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
      // JO, JNO, JB, JNB, JE, JNE, JBE, JNBE
      let second = iter.next()?;
      match first_octet {
        0 => Op::Jo(second),
        1 => Op::Jno(second),
        2 => Op::Jb(second),
        3 => Op::Jae(second),
        4 => Op::Je(second),
        5 => Op::Jne(second),
        6 => Op::Jbe(second),
        7 => Op::Ja(second),
        _ => return None,
      }
    },
    0x78 => {
      // JS, JNS, JP, JNP, JL, JNL, JLE, JNLE
      let second = iter.next()?;
      match first_octet {
        0 => Op::Js(second),
        1 => Op::Jns(second),
        2 => Op::Jp(second),
        3 => Op::Jnp(second),
        4 => Op::Jl(second),
        5 => Op::Jge(second),
        6 => Op::Jle(second),
        7 => Op::Jg(second),
        _ => return None,
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
      match first_octet {
        0 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm_byte(second, iter)?;
          let src_dest = OpBinarySrcDestDual::Byte(
            OpBinarySrcDestByte::ImmRm(mod_rm, iter.next()?));
          match (second >> 3) & 0x07 {
            0 => Op::Add(src_dest),
            1 => Op::Or(src_dest),
            2 => Op::Adc(src_dest),
            3 => Op::Sbb(src_dest),
            4 => Op::And(src_dest),
            5 => Op::Sub(src_dest),
            6 => Op::Xor(src_dest),
            7 => Op::Cmp(src_dest),
            _ => return None,
          }
        },
        1 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm_word(second, iter)?;
          let src_dest = OpBinarySrcDestDual::Word(
            OpBinarySrcDestWord::ImmRm(mod_rm, iter_next_u16(iter)?));
          match (second >> 3) & 0x07 {
            0 => Op::Add(src_dest),
            1 => Op::Or(src_dest),
            2 => Op::Adc(src_dest),
            3 => Op::Sbb(src_dest),
            4 => Op::And(src_dest),
            5 => Op::Sub(src_dest),
            6 => Op::Xor(src_dest),
            7 => Op::Cmp(src_dest),
            _ => return None,
          }
        },
        2 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm_byte(second, iter)?;
          let src_dest = OpBinarySrcDestDual::Byte(
            OpBinarySrcDestByte::ImmRm(mod_rm, iter.next()?));
          match (second >> 3) & 0x07 {
            0 => Op::Add(src_dest),
            1 => return None,
            2 => Op::Adc(src_dest),
            3 => Op::Sbb(src_dest),
            4 => return None,
            5 => Op::Sub(src_dest),
            6 => return None,
            7 => Op::Cmp(src_dest),
            _ => return None,
          }
        },
        3 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm_word(second, iter)?;
          let src_dest = OpBinarySrcDestDual::Word(
            OpBinarySrcDestWord::ImmRm(mod_rm, iter_next_u16(iter)?));
          match (second >> 3) & 0x07 {
            0 => Op::Add(src_dest),
            1 => return None,
            2 => Op::Adc(src_dest),
            3 => Op::Sbb(src_dest),
            4 => return None,
            5 => Op::Sub(src_dest),
            6 => return None,
            7 => Op::Cmp(src_dest),
            _ => return None,
          }
        },
        4 => {
          // TEST
          let second = iter.next()?;
          Op::Test(OpModRegRmDual::Byte(parse_mod_reg_rm_byte(second, iter)?))
        },
        5 => {
          // TEST
          let second = iter.next()?;
          Op::Test(OpModRegRmDual::Word(parse_mod_reg_rm_word(second, iter)?))
        },
        6 => {
          // XCHG
          let second = iter.next()?;
          Op::XchgRmReg(OpModRegRmDual::Byte(parse_mod_reg_rm_byte(second, iter)?))
        },
        7 => {
          // XCHG
          let second = iter.next()?;
          Op::XchgRmReg(OpModRegRmDual::Word(parse_mod_reg_rm_word(second, iter)?))
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
      // 8F - POP
      match first_octet {
        0..=3 => {
          // MOV
          Op::Mov(parse_binary_src_dest(first, iter)?)
        },
        4 => {
          // mov r/m16, segreg
          let second = iter.next()?;
          let mod_rm = parse_mod_rm_word(second, iter)?;
          let reg = parse_segment_register((second >> 3) & 0x07)?;
          Op::MovWordSeg(OpDirectionType::RegToRm, mod_rm, reg)
        },
        5 => {
          // lea reg16, r/m16
          let second = iter.next()?;
          Op::Lea(parse_mod_reg_rm_word(second, iter)?)
        },
        6 => {
          // mov segreg, r/m16
          let second = iter.next()?;
          let mod_rm = parse_mod_rm_word(second, iter)?;
          let reg = parse_segment_register((second >> 3) & 0x07)?;
          Op::MovWordSeg(OpDirectionType::RmToReg, mod_rm, reg)
        },
        7 => {
          // pop r/m16 (second 000)
          let second = iter.next()?;
          let mod_rm = parse_mod_rm_word(second, iter)?;
          match (second >> 3) & 0x07 {
            0 => Op::PopRm(mod_rm),
            _ => return None,
          }
        },
        _ => return None,
      }
    },
    0x90 => {
      // 90 - NOP
      // 91..97 - XCHG
      Op::XchgRegAx(parse_register_word(first_octet)?)
    },
    0x98 => {
      // 98 - CBW
      // 99 - CWD
      // 9A - CALL FarProc
      // 9B - WAIT
      // 9C - PUSHF
      // 9D - POPF
      // 9E - SAHF
      // 9F - LAHF
      match first_octet {
        0 => Op::Cbw,
        1 => Op::Cwd,
        2 => Op::CallInterDirect(iter_next_u16(iter)?, iter_next_u16(iter)?),
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
        0 => Op::MovMemToAl(iter_next_u16(iter)?),
        1 => Op::MovMemToAx(iter_next_u16(iter)?),
        2 => Op::MovAlToMem(iter_next_u16(iter)?),
        3 => Op::MovAxToMem(iter_next_u16(iter)?),
        4 => Op::Movs(OpWordByte::Byte),
        5 => Op::Movs(OpWordByte::Word),
        6 => Op::Cmps(OpWordByte::Byte),
        7 => Op::Cmps(OpWordByte::Word),
        _ => return None,
      }
    },
    0xA8 => {
      // A8..A9 - TEST
      // AA..AB - STOS
      // AC..AD - LODS
      // AE..AF - SCAS
      match first_octet {
        0 => Op::TestImmAl(iter.next()?),
        1 => Op::TestImmAx(iter_next_u16(iter)?),
        2 => Op::Stos(OpWordByte::Byte),
        3 => Op::Stos(OpWordByte::Word),
        4 => Op::Lods(OpWordByte::Byte),
        5 => Op::Lods(OpWordByte::Word),
        6 => Op::Scas(OpWordByte::Byte),
        7 => Op::Scas(OpWordByte::Word),
        _ => return None,
      }
    },
    0xB0 => {
      // MOV
      Op::Mov(OpBinarySrcDestDual::Byte(OpBinarySrcDestByte::ImmReg(
        parse_register_byte(first_octet)?, iter.next()?)))
    },
    0xB8 => {
      // MOV
      Op::Mov(OpBinarySrcDestDual::Word(OpBinarySrcDestWord::ImmReg(
        parse_register_word(first_octet)?, iter_next_u16(iter)?)))
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
        4 => Op::Les(parse_mod_reg_rm_word(iter.next()?, iter)?),
        5 => Op::Lds(parse_mod_reg_rm_word(iter.next()?, iter)?),
        6 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm_byte(second, iter)?;
          match (second >> 3) & 0x07 {
            0 => Op::Mov(OpBinarySrcDestDual::Byte(
              OpBinarySrcDestByte::ImmRm(mod_rm, iter.next()?))),
            _ => return None,
          }
        },
        7 => {
          let second = iter.next()?;
          let mod_rm = parse_mod_rm_word(second, iter)?;
          match (second >> 3) & 0x07 {
            0 => Op::Mov(OpBinarySrcDestDual::Word(
              OpBinarySrcDestWord::ImmRm(mod_rm, iter_next_u16(iter)?))),
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
          let rotate_type = match (first >> 1) & 0x01 {
            0 => OpRotateType::One,
            1 => OpRotateType::Cl,
            _ => return None,
          };
          let rm = match first & 0x01 {
            0 => OpModRmDual::Byte(parse_mod_rm_byte(second, iter)?),
            1 => OpModRmDual::Word(parse_mod_rm_word(second, iter)?),
            _ => return None,
          };
          match (second >> 3) & 0x07 {
            0 => Op::Rol(rotate_type, rm),
            1 => Op::Ror(rotate_type, rm),
            2 => Op::Rcl(rotate_type, rm),
            3 => Op::Rcr(rotate_type, rm),
            4 => Op::Shl(rotate_type, rm),
            5 => Op::Shr(rotate_type, rm),
            6 => return None,
            7 => Op::Sar(rotate_type, rm),
            _ => return None,
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
      let rm = parse_mod_rm_word(second, iter)?;
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
        0 => Op::Loopnz(iter.next()?),
        1 => Op::Loopz(iter.next()?),
        2 => Op::Loop(iter.next()?),
        3 => Op::Jcxz(iter.next()?),
        4 => Op::InVariable(OpWordByte::Byte, iter.next()?),
        5 => Op::InVariable(OpWordByte::Word, iter.next()?),
        6 => Op::OutVariable(OpWordByte::Byte, iter.next()?),
        7 => Op::OutVariable(OpWordByte::Word, iter.next()?),
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
        0 => Op::CallWithinDirect(iter_next_u16(iter)?),
        1 => Op::JmpWithinDirect(iter_next_u16(iter)?),
        2 => Op::JmpInterDirect(iter_next_u16(iter)?, iter_next_u16(iter)?),
        3 => Op::JmpWithinDirectShort(iter.next()?),
        4 => Op::InFixed(OpWordByte::Byte),
        5 => Op::InFixed(OpWordByte::Word),
        6 => Op::OutFixed(OpWordByte::Byte),
        7 => Op::OutFixed(OpWordByte::Word),
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
        2 => Op::RepUnset,
        3 => Op::RepSet,
        4 => Op::Hlt,
        5 => Op::Cmc,
        6..=7 => {
          let second = iter.next()?;
          let rm = match first & 0x01 {
            0 => OpModRmDual::Byte(parse_mod_rm_byte(second, iter)?),
            1 => OpModRmDual::Word(parse_mod_rm_word(second, iter)?),
            _ => return None,
          };
          match (second >> 3) & 0x07 {
            0 => match rm {
              OpModRmDual::Byte(rm_raw) =>
                Op::TestImmByte(rm_raw, iter.next()?),
              OpModRmDual::Word(rm_raw) =>
                Op::TestImmWord(rm_raw, iter_next_u16(iter)?),
            },
            1 => return None,
            2 => Op::Not(rm),
            3 => Op::Neg(rm),
            4 => Op::Mul(rm),
            5 => Op::Imul(rm),
            6 => Op::Div(rm),
            7 => Op::Idiv(rm),
            _ => return None,
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
      // INC, DEC, CALL, CALL, JMP, JMP, PUSH, -
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
          let rm = parse_mod_rm_byte(second, iter)?;
          match (second >> 3) & 0x07 {
            0 => Op::IncRm(OpModRmDual::Byte(rm)),
            1 => Op::DecRm(OpModRmDual::Byte(rm)),
            _ => return None,
          }
        },
        7 => {
          let second = iter.next()?;
          let rm = parse_mod_rm_word(second, iter)?;
          match (second >> 3) & 0x07 {
            0 => Op::IncRm(OpModRmDual::Word(rm)),
            1 => Op::DecRm(OpModRmDual::Word(rm)),
            2 => Op::CallWithinIndirect(rm),
            3 => Op::CallInterIndirect(rm),
            4 => Op::JmpWithinIndirect(rm),
            5 => Op::JmpInterIndirect(rm),
            6 => Op::PushRm(rm),
            _ => return None,
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
    let input: Vec<u8> = vec![0x00, 0x00];
    assert_eq!(
      parse_op(&mut input.into_iter()),
      Some(Op::Add(OpBinarySrcDestDual::Byte(OpBinarySrcDestByte::RegToRm(
        OpModRegRm::<OpRegisterByte>(
          OpRegisterByte::Al,
          OpModRmByte::Register(OpRegisterByte::Al)))
      ))),
    );
  }
  {
    let input: Vec<u8> = vec![0x06];
    assert_eq!(
      parse_op(&mut input.into_iter()),
      Some(Op::PushSeg(OpSegmentRegister::Es)),
    );
  }
}
