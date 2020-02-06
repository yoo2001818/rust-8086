use super::cpu::CPU;
use super::operand::*;
use super::op::*;

trait OperandOpValue: OperandValue {
  fn exec_binary(
    cpu: &mut CPU,
    op: &OpBinaryOp,
    src: &Operand,
    dest: &Operand,
  ) -> ();
  fn exec_unary(
    cpu: &mut CPU,
    op: &OpUnaryOp,
    src: &Operand,
    dest: &Operand,
  ) -> ();
}

impl OperandOpValue for u8 {
  fn exec_binary(
    cpu: &mut CPU,
    op: &OpBinaryOp,
    src: &Operand,
    dest: &Operand,
  ) -> () {
    let src_val = cpu.get_operand::<u8>(src);
    let dest_val = cpu.get_operand::<u8>(dest);
    match op {
      OpBinaryOp::Adc => {
        let result_val = src_val + dest_val;
        let cf = result_val > src_val && result_val > dest_val;
        let pf = result_val & 0x1 != 0;
        let af = (~(src_val ^ dest_val)) & (src_val ^ result_val) & 0x8;
        let zf = result_val == 0;
        let sf = result_val & 0x80 != 0;
        let of = (~(src_val ^ dest_val)) & (src_val ^ result_val) & 0x80;
        let flagClear = 0b100011010101;
        let flagSet =
          cf ? 0x1 : 0 +
          pf ? 0x2 : 0 +
          af ? 0x8 : 0 +
          zf ? 0x20 : 0 +
          sf ? 0x40 : 0 +
          of ? 0x0800 : 0;
      }
    }
  }
  fn exec_unary(
    cpu: &mut CPU,
    op: &OpUnaryOp,
    src: &Operand,
    dest: &Operand,
  ) -> () {

  }
}

impl OperandOpValue for u16 {
  fn exec_binary(
    cpu: &mut CPU,
    op: &OpBinaryOp,
    src: &Operand,
    dest: &Operand,
  ) -> () {

  }
  fn exec_unary(
    cpu: &mut CPU,
    op: &OpUnaryOp,
    src: &Operand,
    dest: &Operand,
  ) -> () {

  }
}

fn exec_binary_sized(
  cpu: &mut CPU,
  op: &OpBinaryOp,
  size: &OpSize,
  src: &Operand,
  dest: &Operand,
) -> () {
  match size {
    OpSize::Byte => u8::exec_binary(cpu, op, src, dest),
    OpSize::Word => u16::exec_binary(cpu, op, src, dest),
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
