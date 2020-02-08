use super::cpu::CPU;

impl CPU {
  fn get_flags(&self) -> u16 {
    self.register.flags
  }
  fn set_flags(&mut self, value: u16) -> () {
    self.register.flags = value;
  }
  fn blit_flags(&mut self, clear: u16, set: u16) -> () {
    self.register.flags =
      (self.register.flags & !clear) | set;
  }
}

pub const CF: u16 = 0x0001;
pub const PF: u16 = 0x0004;
pub const AF: u16 = 0x0010;
pub const ZF: u16 = 0x0040;
pub const SF: u16 = 0x0080;
pub const OF: u16 = 0x0800;
