struct Cpu {
  opcode: u16,
  memory: [u8, ..4096],
  v_reg: [u8, ..16],
  i_reg: u16,
  pc_reg: u16,
  stack: [u16, ..16],
  sp: u16,
  delay_timer: u8,
  sound_timer: u8,
  keypad: [u8, ..16]
}

impl Cpu {
  pub fn new() -> Cpu {
    let mut cpu = Cpu {
      opcode: 0,
      memory: [0, ..4096],
      v_reg: [0, ..16],
      i_reg: 0,
      pc_reg: 0x200,
      stack: [0, ..16],
      sp: 0,
      delay_timer: 0,
      sound_timer: 0,
      keypad: [0, ..16]
    };

    for i in range(0, 80) { cpu.memory[i] = fontset[i]; }
    cpu
  }

  pub fn emulate_cycle(&mut self) {
    self.fetch_opcode();
    self.opcode_execute();
  }

  fn fetch_opcode(&mut self) {
    self.opcode = (self.memory[self.pc_reg] << 8 | self.memory[self.pc_reg + 1]) as u16;
  }

  fn opcode_execute(&self) {
    match (self.opcode & 0xf000) {
      0x0000 => self.op_0xxx(),
      0x1000 => self.op_1xxx(),
      0x2000 => self.op_2xxx(),
      0x3000 => self.op_3xxx(),
      0x4000 => self.op_4xxx(),
      0x5000 => self.op_5xxx(),
      0x6000 => self.op_6xxx(),
      0x7000 => self.op_7xxx(),
      0x8000 => self.op_8xxx(),
      0x9000 => self.op_9xxx(),
      0xA000 => self.op_Axxx(),
      0xB000 => self.op_Bxxx(),
      0xC000 => self.op_Cxxx(),
      0xD000 => self.op_Dxxx(),
      0xE000 => self.op_Exxx(),
      0xF000 => self.op_Fxxx(),
      _      => println(fmt!("dunno"))
    }
  }

  fn op_0xxx(&self) {}
  fn op_1xxx(&self) {}
  fn op_2xxx(&self) {}
  fn op_3xxx(&self) {}
  fn op_4xxx(&self) {}
  fn op_5xxx(&self) {}
  fn op_6xxx(&self) {}
  fn op_7xxx(&self) {}
  fn op_8xxx(&self) {}
  fn op_9xxx(&self) {}
  fn op_Axxx(&self) {}
  fn op_Bxxx(&self) {}
  fn op_Cxxx(&self) {}
  fn op_Dxxx(&self) {}
  fn op_Exxx(&self) {}
  fn op_Fxxx(&self) {}
}

static fontset: [u8, ..80] = [0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70,
                              0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0,
                              0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0,
                              0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40,
                              0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0,
                              0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0,
                              0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
                              0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80];
