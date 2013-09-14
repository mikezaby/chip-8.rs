use std::run;

struct Display {
  gfx: [[u8, ..64], ..32],
  draw_flag: bool,
  screen: ~str
}

impl Display {
  pub fn new() -> Display {
    Display {
      gfx: [[0, ..64], ..32],
      draw_flag: true,
      screen: ~""
    }
  }

  pub fn clear(&mut self) {
    self.gfx = [[0, ..64], ..32];
    self.draw_flag = true;
  }

  pub fn draw(&mut self, x: u8, y: u8, sprite: &[u8]) -> u8 {
    let mut collision = 0u8;
    let n = sprite.len() as u8;
    for j in range(0u8, n) {
      for i in range(0u8, 8) {
        if (sprite[j] & (0x80 >> i)) != 0 {
          if self.gfx[y + j][x + i] == 1 { collision = 1 }
          self.gfx[y + j][x + i] ^= 1;
        }
      }
    }

    self.draw_flag = true;
    collision
  }

  pub fn draw_screen(&mut self) {
    if !self.draw_flag { return }
    let mut pixel: char;
    self.screen = ~"";

    for y in range(0, 32) {
      for x in range(0, 64) {
        pixel = if self.gfx[y][x] != 0 { '█' } else { ' ' };
        self.screen.push_char(pixel);
      }
      self.screen.push_str("\n");
    }
    run::process_status("clear", []);
    println(self.screen);

    self.draw_flag = false;
  }
}
