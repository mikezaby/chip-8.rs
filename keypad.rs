use sdl::event::*;

struct Keypad {
  keys: [bool, ..16]
}

impl Keypad {
  pub fn new() -> Keypad {
    Keypad { keys: [false, ..16] }
  }

  pub fn pressed(self, index: u8) -> bool {
    self.keys[index]
  }

  pub fn press(&mut self, key: Key, state: bool) {
    match key {
      Num1Key => self.set_key(0x1, state),
      Num2Key => self.set_key(0x2, state),
      Num3Key => self.set_key(0x3, state),
      Num4Key => self.set_key(0xc, state),
      QKey    => self.set_key(0x4, state),
      WKey    => self.set_key(0x5, state),
      EKey    => self.set_key(0x6, state),
      RKey    => self.set_key(0xd, state),
      AKey    => self.set_key(0x7, state),
      SKey    => self.set_key(0x8, state),
      DKey    => self.set_key(0x9, state),
      FKey    => self.set_key(0xe, state),
      ZKey    => self.set_key(0xa, state),
      XKey    => self.set_key(0x0, state),
      CKey    => self.set_key(0xb, state),
      VKey    => self.set_key(0xf, state),
      _       => ()
    }
  }

  fn set_key(&mut self, index: u8, state: bool) {
    self.keys[index] = state;
  }
}
