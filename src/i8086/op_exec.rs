use super::cpu::CPU;
use super::operand::*;
use super::op::*;
use super::register::*;
use super::flags::*;

type Flags = (u16, u16);

trait OperandOpValue: Sized + Copy {
  fn zero() -> Self;
  fn get_stack_size() -> u16;
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
  fn not(src: Self) -> Self;
  fn mul(cpu: &mut CPU, value: Self) -> ();
  fn imul(cpu: &mut CPU, value: Self) -> ();
  fn div(cpu: &mut CPU, value: Self) -> Option<()>;
  fn idiv(cpu: &mut CPU, value: Self) -> Option<()>;
  fn get_flags(value: Self) -> Flags;
}

impl OperandOpValue for u8 {
  fn zero() -> u8 { 0 }
  fn get_stack_size() -> u16 { 1 }
  fn add(src: u8, dest: u8, carry: bool) -> (u8, Flags) {
    let result = src + dest + (if carry { 1 } else { 0 });
    let cf = result > src && result > dest;
    let af = (!(src ^ dest)) & (src ^ result) & 0x8 != 0;
    let of = (!(src ^ dest)) & (src ^ result) & 0x80 != 0;
    let (prev_clear, prev_set) = OperandOpValue::get_flags(result);
    (result, (
      prev_clear | CF | AF | OF,
      prev_set |
      if cf { CF } else { 0 } |
      if af { AF } else { 0 } |
      if of { OF } else { 0 },
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
      prev_clear | CF | AF | OF,
      prev_set |
      if cf { CF } else { 0 } |
      if af { AF } else { 0 } |
      if of { OF } else { 0 },
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
  fn not(dest: u8) -> u8 {
    !dest
  }
  fn mul(cpu: &mut CPU, value: u8) -> () {
    let other = u8::read_reg(&cpu.register, &RegisterByteType::Al);
    let result = (other as u16) * (value as u16);
    u16::write_reg(&mut cpu.register, &RegisterWordType::Ax, result);
    cpu.blit_flags(OF | CF, if result & 0xFF00 == 0 { OF | CF } else { 0 });
  }
  fn imul(cpu: &mut CPU, value: u8) -> () {
    let other = u8::read_reg(&cpu.register, &RegisterByteType::Al);
    let result = ((other as i8 as i16) * (value as i8 as i16)) as u16;
    u16::write_reg(&mut cpu.register, &RegisterWordType::Ax, result);
    cpu.blit_flags(OF | CF, if result & 0xFF00 == 0 { OF | CF } else { 0 });
  }
  fn div(cpu: &mut CPU, value: u8) -> Option<()> {
    if value == 0 {
      return None;
    }
    let dividend = u16::read_reg(&cpu.register, &RegisterWordType::Ax);
    let quotient = dividend / (value as u16);
    if quotient > 0xFF {
      return None;
    }
    let remainder = dividend % (value as u16);
    u8::write_reg(&mut cpu.register, &RegisterByteType::Al, quotient as u8);
    u8::write_reg(&mut cpu.register, &RegisterByteType::Ah, remainder as u8);
    Some(())
  }
  fn idiv(cpu: &mut CPU, value: u8) -> Option<()> {
    if value == 0 {
      return None;
    }
    let dividend = u16::read_reg(&cpu.register, &RegisterWordType::Ax);
    let quotient = (dividend as i16 / (value as i16)) as u16;
    if quotient > 0x7F || quotient < 0x80 {
      return None;
    }
    let remainder = ((dividend as i16) % (value as i16)) as u16;
    u8::write_reg(&mut cpu.register, &RegisterByteType::Al, quotient as u8);
    u8::write_reg(&mut cpu.register, &RegisterByteType::Ah, remainder as u8);
    Some(())
  }
  fn get_flags(value: u8) -> Flags {
    let sf = value & 0x80 != 0;
    let zf = value == 0;
    let mut pf = true;
    for i in 0..8 {
      pf = pf != ((value & (1 << i)) != 0);
    }
    (SF | ZF | PF,
      if sf { SF } else { 0 } |
      if zf { ZF } else { 0 } |
      if pf { PF } else { 0 }
    )
  }
}

fn exec_binary<T, R>(
  cpu: &mut CPU,
  op: &OpBinaryOp,
  src: &Operand<R>,
  dest: &Operand<R>,
) -> () 
  where T: OperandValue<R> + OperandOpValue, R: RegisterType
{
  let src_val: T = cpu.get_operand(src);
  let dest_val: T = cpu.get_operand(dest);
  match op {
    OpBinaryOp::Adc => {
      let has_carry = cpu.get_flags() & CF != 0;
      let (result, (flag_clear, flag_set)) =
        OperandOpValue::add(src_val, dest_val, has_carry);
      cpu.blit_flags(flag_clear, flag_set);
      cpu.set_operand(dest, result);
    },
    OpBinaryOp::Add => {
      let (result, (flag_clear, flag_set)) =
        OperandOpValue::add(src_val, dest_val, false);
      cpu.blit_flags(flag_clear, flag_set);
      cpu.set_operand(dest, result);
    },
    OpBinaryOp::And => {
      let result = OperandOpValue::and(src_val, dest_val);
      let (flag_clear, flag_set) = OperandOpValue::get_flags(result);
      cpu.blit_flags(flag_clear | OF | CF, flag_set);
      cpu.set_operand(dest, result);
    },
    OpBinaryOp::Cmp => {
      let (result, (flag_clear, flag_set)) =
        OperandOpValue::sub(src_val, dest_val, false);
      cpu.blit_flags(flag_clear, flag_set);
    },
    OpBinaryOp::Mov => {
      cpu.set_operand(dest, src_val);
    },
    OpBinaryOp::Or => {
      let result = OperandOpValue::or(src_val, dest_val);
      let (flag_clear, flag_set) = OperandOpValue::get_flags(result);
      cpu.blit_flags(flag_clear | OF | CF, flag_set);
      cpu.set_operand(dest, result);
    },
    OpBinaryOp::Sbb => {
      let has_carry = cpu.get_flags() & CF != 0;
      let (result, (flag_clear, flag_set)) =
        OperandOpValue::sub(src_val, dest_val, has_carry);
      cpu.blit_flags(flag_clear, flag_set);
      cpu.set_operand(dest, result);
    },
    OpBinaryOp::Sub => {
      let (result, (flag_clear, flag_set)) =
        OperandOpValue::sub(src_val, dest_val, false);
      cpu.blit_flags(flag_clear, flag_set);
      cpu.set_operand(dest, result);
    },
    OpBinaryOp::Test => {
      let result = OperandOpValue::and(src_val, dest_val);
      let (flag_clear, flag_set) = OperandOpValue::get_flags(result);
      cpu.blit_flags(flag_clear | OF | CF, flag_set);
    },
    OpBinaryOp::Xchg => {
      cpu.set_operand(dest, src_val);
      cpu.set_operand(src, dest_val);
    },
    OpBinaryOp::Xor => {
      let result = OperandOpValue::xor(src_val, dest_val);
      let (flag_clear, flag_set) = OperandOpValue::get_flags(result);
      cpu.blit_flags(flag_clear | OF | CF, flag_set);
      cpu.set_operand(dest, result);
    },
  }
}

fn exec_unary<T, R>(
  cpu: &mut CPU,
  op: &OpUnaryOp,
  dest: &Operand<R>,
) -> () 
  where T: OperandValue<R> + OperandOpValue, R: RegisterType
{
  let dest_val: T = cpu.get_operand(dest);
  match op {
    OpUnaryOp::Push => {
      // TODO Is this really good idea?
      cpu.register.sp -= T::get_stack_size();
      cpu.set_operand_with_seg::<T, R>(
        &Operand::Direct(cpu.register.sp),
        &Some(RegisterWordType::Sp),
        dest_val);
    },
    OpUnaryOp::Pop => {
      // TODO Is this really good idea?
      cpu.register.sp += T::get_stack_size();
      let result = cpu.get_operand_with_seg::<T, R>(
        &Operand::Direct(cpu.register.sp),
        &Some(RegisterWordType::Sp));
      cpu.set_operand(dest, result);
    },
    OpUnaryOp::Inc => {
      let (result, (flag_clear, flag_set)) =
        OperandOpValue::add(OperandOpValue::zero(), dest_val, true);
      cpu.blit_flags(flag_clear, flag_set);
      cpu.set_operand(dest, result);
    },
    OpUnaryOp::Dec => {
      let (result, (flag_clear, flag_set)) =
        OperandOpValue::sub(OperandOpValue::zero(), dest_val, true);
      cpu.blit_flags(flag_clear, flag_set);
      cpu.set_operand(dest, result);
    },
    OpUnaryOp::Not => {
      cpu.set_operand(dest, OperandOpValue::not(dest_val));
    },
    OpUnaryOp::Neg => {
      let (result, (flag_clear, flag_set)) =
        OperandOpValue::sub(dest_val, OperandOpValue::zero(), false);
      cpu.blit_flags(flag_clear, flag_set);
      cpu.set_operand(dest, result);
    },
    OpUnaryOp::Mul => {
      OperandOpValue::mul(cpu, dest_val);
    },
    OpUnaryOp::Imul => {
      OperandOpValue::imul(cpu, dest_val);
    },
    OpUnaryOp::Div => {
      OperandOpValue::div(cpu, dest_val);
    },
    OpUnaryOp::Idiv => {
      OperandOpValue::idiv(cpu, dest_val);
    },
  }
}

impl CPU {
  pub fn exec_op(&mut self, op: &Op) -> () {
    match op {
      Op::BinaryByte { op, src, dest } => {
        exec_binary::<u8, RegisterByteType>(self, op, src, dest);
      },
      Op::BinaryWord { op, src, dest } => {
        // exec_binary::<u16, RegisterWordType>(self, op, src, dest);
      },
      Op::UnaryByte { op, dest } => {
        exec_unary::<u8, RegisterByteType>(self, op, dest);
      },
      Op::UnaryWord { op, dest } => {},
      Op::Nullary(op) => {},
      Op::ShiftByte { op, shift_type, dest } => {},
      Op::ShiftWord { op, shift_type, dest } => {},
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
