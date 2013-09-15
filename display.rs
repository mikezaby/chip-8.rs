use sdl::video::*;
use sdl::Rect;

struct Display {
  gfx: [[u8, ..64], ..32],
  draw_flag: bool,
  screen: ~Surface
}

static scale: int = 20;

impl Display {
  pub fn new() -> Display {
    Display {
      gfx: [[0, ..64], ..32],
      draw_flag: true,
      screen: set_video_mode(64*scale, 32*scale, 8, [HWSurface], [DoubleBuf]).unwrap()
    }
  }

  pub fn clear(&mut self) {
    self.gfx = [[0, ..64], ..32];
    self.draw_flag = true;
  }

  pub fn draw(&mut self, x: u8, y: u8, sprite: &[u8]) -> u8 {
    let mut collision = 0u8;
    let n = sprite.len() as u8;
    let mut yj: u8;
    let mut xi: u8;

    for j in range(0u8, n) {
      for i in range(0u8, 8) {
        yj = (y + j) % 32;
        xi = (x + i) % 64;

        if (sprite[j] & (0x80 >> i)) != 0 {
          if self.gfx[yj][xi] == 1 { collision = 1 }
          self.gfx[yj][xi] ^= 1;
        }
      }
    }

    self.draw_flag = true;
    collision
  }

  pub fn draw_screen(&mut self) {
    if !self.draw_flag { return }
    let mut pixel: u8;
    let sc = scale as u16;
    let pt = |p: i16| { p * (scale as i16) };

    for y in range(0i16, 32) {
      for x in range(0i16, 64) {
        pixel = if self.gfx[y][x] != 0 { 255 } else { 0 };
        self.screen.fill_rect(Some(Rect { x: pt(x), y: pt(y), w: sc, h: sc}),
                              RGB(pixel, pixel, pixel));
      }
    }

    self.screen.flip();
    self.draw_flag = false;
  }
}
