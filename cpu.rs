struct Cpu {
  opcode: u16,
  memory: [u8, ..4096],
  v_reg: [u8, ..16],
  i_reg: u16,
  pc_reg: u16
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      opcode: 0,
      memory: [0, ..4096],
      v_reg: [0, ..16],
      i_reg: 0,
      pc_reg: 0x200
    }
  }

  pub fn emulate_cycle(&self) {}
}
