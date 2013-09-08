struct Display {
  gfx: [[u8, ..64], ..32],
  draw_flag: bool
}

impl Display {
  pub fn new() -> Display {
    Display {
      gfx: [[0, ..64], ..32],
      draw_flag: true
    }
  }

  pub fn clear(&mut self) {
    self.gfx = [[0, ..64], ..32];
    self.draw_flag = true;
  }

  pub fn draw(&mut self, x: u8, y: u8, sprite: &[u8]) -> u8 {
    let mut collision = 0u8;
    let n = sprite.len();

    for j in range(0u, n) {
      for i in range(0u, 8) {
        if (sprite[j] & (0x80 >> i)) != 0 {
          if collision == 0 && self.gfx[y][x] == 1 { collision = 1 }
          self.gfx[y][x] ^= 1;
        }
      }
    }

    self.draw_flag = true;
    collision
  }
}
