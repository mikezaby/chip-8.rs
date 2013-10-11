use sdl::event;

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

  pub fn press(&mut self, key: event::Key, state: bool) {
    match key {
      event::Num1Key => self.set_key(0x1, state),
      event::Num2Key => self.set_key(0x2, state),
      event::Num3Key => self.set_key(0x3, state),
      event::Num4Key => self.set_key(0xc, state),
      event::QKey    => self.set_key(0x4, state),
      event::WKey    => self.set_key(0x5, state),
      event::EKey    => self.set_key(0x6, state),
      event::RKey    => self.set_key(0xd, state),
      event::AKey    => self.set_key(0x7, state),
      event::SKey    => self.set_key(0x8, state),
      event::DKey    => self.set_key(0x9, state),
      event::FKey    => self.set_key(0xe, state),
      event::ZKey    => self.set_key(0xa, state),
      event::XKey    => self.set_key(0x0, state),
      event::CKey    => self.set_key(0xb, state),
      event::VKey    => self.set_key(0xf, state),
      _       => ()
    }
  }

  fn set_key(&mut self, index: u8, state: bool) {
    self.keys[index] = state;
  }
}
