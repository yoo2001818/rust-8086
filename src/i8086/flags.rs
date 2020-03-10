use super::cpu::CPU;

impl CPU {
  pub fn get_flags(&self) -> u16 {
    self.register.flags
  }
  pub fn set_flags(&mut self, value: u16) -> () {
    self.register.flags = value;
  }
  pub fn blit_flags(&mut self, clear: u16, set: u16) -> () {
    self.register.flags =
      (self.register.flags & !clear) | set;
  }
}

pub const CF: u16 = 0x0001;
pub const PF: u16 = 0x0004;
pub const AF: u16 = 0x0010;
pub const ZF: u16 = 0x0040;
pub const SF: u16 = 0x0080;
pub const TF: u16 = 0x0100;
pub const IF: u16 = 0x0200;
pub const DF: u16 = 0x0400;
pub const OF: u16 = 0x0800;
