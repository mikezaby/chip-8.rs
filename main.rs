use cpu::Cpu;
use std::io;

pub mod cpu;

fn main() {
  let mut cpu = Cpu::new();
  let game = io::stdin().read_line();
  cpu.load_game(~"games/" + game);

  loop {
    cpu.emulate_cycle();
    cpu.display.draw_screen();
  }
}
