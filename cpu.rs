struct Cpu {
  opcode: u16,
  memory: ~[u8],
  v_reg: ~[u8],
  i_reg: u8,
  pc_reg: u8
}

pub impl Cpu {
  pub fn initialize() {}

  pub fn emulate_cycle() {}
}
