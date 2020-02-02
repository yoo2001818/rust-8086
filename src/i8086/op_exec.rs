use super::cpu::CPU;
use super::operand::*;
use super::op::*;

fn exec_binary(
  cpu: &mut CPU,
  op: &OpBinaryOp,
  size: &OpSize,
  src: &Operand,
  dest: &Operand,
) -> () {
  cpu.get_operand_u16(src);
  cpu.get_operand_u16(dest);
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
