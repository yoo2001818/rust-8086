use super::cpu::CPU;
use super::operand::*;
use super::op::*;

type Flags = (u16, u16);

trait OperandOpValue: Sized {
  fn add(
    src: Self,
    dest: Self,
    carry: bool,
  ) -> (Self, Flags);
  fn sub(
    src: Self,
    dest: Self,
    carry: bool,
  ) -> (Self, Flags);
  fn and(src: Self, dest: Self) -> Self;
  fn or(src: Self, dest: Self) -> Self;
  fn xor(src: Self, dest: Self) -> Self;
  fn get_flags(value: Self) -> Flags;
}

impl OperandOpValue for u8 {
  fn add(src: u8, dest: u8, carry: bool) -> (u8, Flags) {
    let result = src + dest + (if carry { 1 } else { 0 });
    let cf = result > src && result > dest;
    let af = (!(src ^ dest)) & (src ^ result) & 0x8 != 0;
    let of = (!(src ^ dest)) & (src ^ result) & 0x80 != 0;
    let (prev_clear, prev_set) = OperandOpValue::get_flags(result);
    (result, (
      prev_clear | 0x0809,
      prev_set |
      if cf { 0x1 } else { 0 } |
      if af { 0x8 } else { 0 } |
      if of { 0x0800 } else { 0 },
    ))
  }
  fn sub(src: u8, dest: u8, carry: bool) -> (u8, Flags) {
    let new_src = (-(src as i8) - (if carry { 1 } else { 0 })) as u8;
    let result = dest - new_src;
    let cf = result > new_src && result > dest;
    let af = (!(new_src ^ dest)) & (new_src ^ result) & 0x8 != 0;
    let of = (!(new_src ^ dest)) & (new_src ^ result) & 0x80 != 0;
    let (prev_clear, prev_set) = OperandOpValue::get_flags(result);
    (result, (
      prev_clear | 0x0809,
      prev_set |
      if cf { 0x1 } else { 0 } |
      if af { 0x8 } else { 0 } |
      if of { 0x0800 } else { 0 },
    ))
  }
  fn and(src: u8, dest: u8) -> u8 {
    src & dest
  }
  fn or(src: u8, dest: u8) -> u8 {
    src | dest
  }
  fn xor(src: u8, dest: u8) -> u8 {
    src ^ dest
  }
  fn get_flags(value: u8) -> Flags {
    let sf = value & 0x80 != 0;
    let zf = value == 0;
    let pf = value & 0x1 != 0;
    (0x62,
      if sf { 0x40 } else { 0 } |
      if zf { 0x20 } else { 0 } |
      if pf { 0x2 } else { 0 }
    )
  }
}

fn exec_binary<R, V>(
  cpu: &mut CPU,
  op: &OpBinaryOp,
  src: &Operand<R>,
  dest: &Operand<R>,
) -> () {
  match op {
    OpBinaryOp::Adc => (),
    OpBinaryOp::Add => (),
    OpBinaryOp::And => (),
    OpBinaryOp::Cmp => (),
    OpBinaryOp::Mov => (),
    OpBinaryOp::Or => (),
    OpBinaryOp::Sbb => (),
    OpBinaryOp::Sub => (),
    OpBinaryOp::Test => (),
    OpBinaryOp::Xchg => (),
    OpBinaryOp::Xor => (),
  }
}

impl CPU {
  pub fn exec_op(&mut self, op: &Op) -> () {
    match op {
      Op::Binary { op, size, src, dest } =>
        exec_binary(self, op, size, src, dest),
      Op::Unary { op, size, dest } =>
        exec_unary(self, op, size, dest),
      Op::Nullary(op) => {},
      Op::Shift { op, shift_type, size, dest } => {},
      Op::CondJmp { op, offset } => {},
      Op::InFixed(size) => {},
      Op::InVariable(size, value) => {},
      Op::OutFixed(size) => {},
      Op::OutVariable(size, value) => {},
      Op::Lea(reg, operand) => {},
      Op::Lds(reg, operand) => {},
      Op::Les(reg, operand) => {},
      Op::Movs(size) => {},
      Op::Cmps(size) => {},
      Op::Scas(size) => {},
      Op::Lods(size) => {},
      Op::Stos(size) => {},
      Op::Call(call_type) => {},
      Op::Jmp(call_type) => {},
      Op::RetWithin => {},
      Op::RetWithinImm(value) => {},
      Op::RetInter => {},
      Op::RetInterImm(value) => {},
      Op::Int(value) => {},
      Op::Esc(code, operand) => {},
      Op::Segment(seg) => {},
    }
  }
}
