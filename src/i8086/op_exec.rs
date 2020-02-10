use super::cpu::CPU;
use super::operand::*;
use super::op::*;
use super::register::*;
use super::flags::*;

type Flags = (u16, u16);

trait OperandOpValue: Sized + Copy {
  fn zero() -> Self;
  fn one() -> Self;
  fn get_stack_size() -> u16;
  fn get_bits() -> u8;
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
  fn shl(src: Self, count: u8) -> Self;
  fn sar(src: Self, count: u8) -> Self;
  fn shr(src: Self, count: u8) -> Self;
  fn msb(src: Self) -> bool;
  fn lsb(src: Self) -> bool;
  fn get_flags(value: Self) -> Flags;
}

impl OperandOpValue for u8 {
  fn zero() -> u8 { 0 }
  fn one() -> u8 { 1 }
  fn get_stack_size() -> u16 { 2 }
  fn get_bits() -> u8 { 8 }
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
  fn shl(src: u8, count: u8) -> u8 {
    src << count
  }
  fn shr(src: u8, count: u8) -> u8 {
    src >> count
  }
  fn sar(src: u8, count: u8) -> u8 {
    ((src as i8) >> count) as u8
  }
  fn msb(src: u8) -> bool {
    src & 0x80 != 0
  }
  fn lsb(src: u8) -> bool {
    src & 0x1 != 0
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

impl OperandOpValue for u16 {
  fn zero() -> u16 { 0 }
  fn one() -> u16 { 1 }
  fn get_stack_size() -> u16 { 2 }
  fn get_bits() -> u8 { 16 }
  fn add(src: u16, dest: u16, carry: bool) -> (u16, Flags) {
    let result = src + dest + (if carry { 1 } else { 0 });
    let cf = result > src && result > dest;
    let af = (!(src ^ dest)) & (src ^ result) & 0x8 != 0;
    let of = (!(src ^ dest)) & (src ^ result) & 0x8000 != 0;
    let (prev_clear, prev_set) = OperandOpValue::get_flags(result);
    (result, (
      prev_clear | CF | AF | OF,
      prev_set |
      if cf { CF } else { 0 } |
      if af { AF } else { 0 } |
      if of { OF } else { 0 },
    ))
  }
  fn sub(src: u16, dest: u16, carry: bool) -> (u16, Flags) {
    let new_src = (-(src as i8) - (if carry { 1 } else { 0 })) as u16;
    let result = dest - new_src;
    let cf = result > new_src && result > dest;
    let af = (!(new_src ^ dest)) & (new_src ^ result) & 0x8 != 0;
    let of = (!(new_src ^ dest)) & (new_src ^ result) & 0x8000 != 0;
    let (prev_clear, prev_set) = OperandOpValue::get_flags(result);
    (result, (
      prev_clear | CF | AF | OF,
      prev_set |
      if cf { CF } else { 0 } |
      if af { AF } else { 0 } |
      if of { OF } else { 0 },
    ))
  }
  fn and(src: u16, dest: u16) -> u16 {
    src & dest
  }
  fn or(src: u16, dest: u16) -> u16 {
    src | dest
  }
  fn xor(src: u16, dest: u16) -> u16 {
    src ^ dest
  }
  fn not(dest: u16) -> u16 {
    !dest
  }
  fn mul(cpu: &mut CPU, value: u16) -> () {
    let other = u16::read_reg(&cpu.register, &RegisterWordType::Ax);
    let result = (other as u32) * (value as u32);
    u16::write_reg(&mut cpu.register, &RegisterWordType::Ax,
      (result & 0xFFFF) as u16);
    u16::write_reg(&mut cpu.register, &RegisterWordType::Dx,
      ((result >> 16) & 0xFFFF) as u16);
    cpu.blit_flags(OF | CF, if result & 0xFFFF0000 == 0 { OF | CF } else { 0 });
  }
  fn imul(cpu: &mut CPU, value: u16) -> () {
    let other = u16::read_reg(&cpu.register, &RegisterWordType::Ax);
    let result = ((other as i8 as i32) * (value as i8 as i32)) as u32;
    u16::write_reg(&mut cpu.register, &RegisterWordType::Ax,
      (result & 0xFFFF) as u16);
    u16::write_reg(&mut cpu.register, &RegisterWordType::Dx,
      ((result >> 16) & 0xFFFF) as u16);
    cpu.blit_flags(OF | CF, if result & 0xFFFF0000 == 0 { OF | CF } else { 0 });
  }
  fn div(cpu: &mut CPU, value: u16) -> Option<()> {
    if value == 0 {
      return None;
    }
    let dividend =
      u16::read_reg(&cpu.register, &RegisterWordType::Ax) as u32 |
      ((u16::read_reg(&cpu.register, &RegisterWordType::Dx) as u32) << 16);
    let quotient = dividend / (value as u32);
    if quotient > 0xFFFF {
      return None;
    }
    let remainder = dividend % (value as u32);
    u16::write_reg(&mut cpu.register, &RegisterWordType::Ax, quotient as u16);
    u16::write_reg(&mut cpu.register, &RegisterWordType::Dx, remainder as u16);
    Some(())
  }
  fn idiv(cpu: &mut CPU, value: u16) -> Option<()> {
    if value == 0 {
      return None;
    }
    let dividend =
      u16::read_reg(&cpu.register, &RegisterWordType::Ax) as u32 |
      ((u16::read_reg(&cpu.register, &RegisterWordType::Dx) as u32) << 16);
    let quotient = (dividend as i32 / (value as i32)) as u32;
    if quotient > 0x7FFF || quotient < 0x8000 {
      return None;
    }
    let remainder = ((dividend as i32) % (value as i32)) as u32;
    u16::write_reg(&mut cpu.register, &RegisterWordType::Ax, quotient as u16);
    u16::write_reg(&mut cpu.register, &RegisterWordType::Dx, remainder as u16);
    Some(())
  }
  fn shl(src: u16, count: u8) -> u16 {
    src << count
  }
  fn shr(src: u16, count: u8) -> u16 {
    src >> count
  }
  fn sar(src: u16, count: u8) -> u16 {
    ((src as i16) >> count) as u16
  }
  fn msb(src: u16) -> bool {
    src & 0x8000 != 0
  }
  fn lsb(src: u16) -> bool {
    src & 0x1 != 0
  }
  fn get_flags(value: u16) -> Flags {
    let sf = value & 0x8000 != 0;
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

fn exec_shift<T, R>(
  cpu: &mut CPU,
  op: &OpShiftOp,
  shift_type: &OpShiftType,
  dest: &Operand<R>,
) -> () 
  where T: OperandValue<R> + OperandOpValue, R: RegisterType
{
  let dest_val: T = cpu.get_operand(dest);
  let count = match shift_type {
    OpShiftType::Cl => u8::read_reg(&cpu.register, &RegisterByteType::Cl),
    OpShiftType::One => 1,
  };
  let (result, (flag_clear, flag_set)) = match op {
    OpShiftOp::Rol => {
      let bits = T::get_bits();
      let wrapped_count = count % bits;
      // a = a << 1 | a >> 7;
      let result = T::or(
        T::shl(dest_val, wrapped_count),
        T::shr(dest_val, bits - wrapped_count));
      let cf = T::lsb(result);
      let of = count == 1 && T::msb(result) != cf;
      (result, (CF | OF, if cf { CF } else { 0 } | if of { OF } else { 0 }))
    },
    OpShiftOp::Ror => {
      let bits = T::get_bits();
      let wrapped_count = count % bits;
      // a = a >> 1 | a << 7;
      let result = T::or(
        T::shr(dest_val, wrapped_count),
        T::shl(dest_val, bits - wrapped_count));
      let cf = T::msb(result);
      let of = count == 1 && cf != T::msb(T::shl(result, 1));
      (result, (CF | OF, if cf { CF } else { 0 } | if of { OF } else { 0 }))
    },
    OpShiftOp::Rcl => {
      let bits = T::get_bits() + 1;
      let wrapped_count = count % (bits + 1);
      let prev_cf = cpu.get_flags() % CF != 0;
      // a = a << 1 | a >> 8 | cf;
      // a = a << 2 | a >> 7 | cf << 1;
      let result = T::or(
        T::shl(if prev_cf { T::one() } else { T::zero() }, wrapped_count - 1),
        T::or(
          T::shl(dest_val, wrapped_count),
          T::shr(dest_val, bits + 1 - wrapped_count)),
        );
      let cf = T::msb(T::shl(dest_val, wrapped_count - 1));
      let of = count == 1 && T::msb(result) != cf;
      (result, (CF | OF, if cf { CF } else { 0 } | if of { OF } else { 0 }))
    },
    OpShiftOp::Rcr => {
      let bits = T::get_bits() + 1;
      let wrapped_count = count % (bits + 1);
      let prev_cf = cpu.get_flags() % CF != 0;
      let of = count == 1 && T::msb(dest_val) != prev_cf;
      // a = a >> 1 | a << 8 | cf << 7;
      // a = a >> 2 | a << 7 | cf << 6;
      let result = T::or(
        T::shl(if prev_cf { T::one() } else { T::zero() }, bits - wrapped_count),
        T::or(
          T::shr(dest_val, wrapped_count),
          T::shl(dest_val, bits + 1 - wrapped_count)),
        );
      let cf = T::lsb(T::shr(dest_val, wrapped_count - 1));
      (result, (CF | OF, if cf { CF } else { 0 } | if of { OF } else { 0 }))
    },
    OpShiftOp::Shl | OpShiftOp::Sal => {
      let cf = T::msb(T::shl(dest_val, count - 1));
      let result = T::shl(dest_val, count);
      let of = count == 1 && T::msb(result) != cf;
      (result, (CF | OF, if cf { CF } else { 0 } | if of { OF } else { 0 }))
    },
    OpShiftOp::Shr => {
      let cf = T::lsb(T::shr(dest_val, count - 1));
      let result = T::shr(dest_val, count);
      let of = count == 1 && T::msb(dest_val);
      (result, (CF | OF, if cf { CF } else { 0 } | if of { OF } else { 0 }))
    },
    OpShiftOp::Sar => {
      let cf = T::lsb(T::sar(dest_val, count - 1));
      let result = T::sar(dest_val, count);
      (result, (CF, if cf { CF } else { 0 }))
    },
  };
  cpu.set_operand(dest, result);
  cpu.blit_flags(flag_clear, flag_set);
}

impl CPU {
  pub fn exec_op(&mut self, op: &Op) -> () {
    match op {
      Op::BinaryByte { op, src, dest } => {
        exec_binary::<u8, RegisterByteType>(self, op, src, dest);
      },
      Op::BinaryWord { op, src, dest } => {
        exec_binary::<u16, RegisterWordType>(self, op, src, dest);
      },
      Op::UnaryByte { op, dest } => {
        exec_unary::<u8, RegisterByteType>(self, op, dest);
      },
      Op::UnaryWord { op, dest } => {
        exec_unary::<u16, RegisterWordType>(self, op, dest);
      },
      Op::ShiftByte { op, shift_type, dest } => {
        exec_shift::<u8, RegisterByteType>(self, op, shift_type, dest);
      },
      Op::ShiftWord { op, shift_type, dest } => {
        exec_shift::<u16, RegisterWordType>(self, op, shift_type, dest);
      },
      Op::Nullary(op) => {},
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
