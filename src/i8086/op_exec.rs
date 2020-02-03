use super::cpu::CPU;
use super::operand::*;
use super::op::*;

fn exec_binary<T: OperandValue>(
  cpu: &mut CPU,
  op: &OpBinaryOp,
  src: &Operand,
  dest: &Operand,
) -> () {
  let src_val: T = cpu.get_operand(src);
  let mut dest_val: T = cpu.get_operand(dest);
  match op {
    OpBinaryOp::Adc => {
      dest_val = src_val + dest_val;
    },
    OpBinaryOp::Add => {
      dest_val = src_val + dest_val;
    },
    _ => (),
  }
  cpu.set_operand(dest, dest_val);
}

fn exec_binary_sized(
  cpu: &mut CPU,
  op: &OpBinaryOp,
  size: &OpSize,
  src: &Operand,
  dest: &Operand,
) -> () {
  let src_val = cpu.get_operand_u16(src);
  let mut dest_val = cpu.get_operand_u16(dest);
  match op {
    OpBinaryOp::Adc => {
      dest_val = src_val + dest_val;
    },
    OpBinaryOp::Add => {
      dest_val = src_val + dest_val;
    },
    _ => (),
  }
  cpu.set_operand_u16(dest, dest_val);
}

fn exec_unary(
  cpu: &mut CPU,
  op: &OpUnaryOp,
  size: &OpSize,
  dest: &Operand,
) -> () {

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
