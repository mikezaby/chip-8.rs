use cpu::Cpu;

pub mod cpu;

fn main() {
  let mut cpu = Cpu::new();
  cpu.load_game(~"games/ibm");
  loop {
    cpu.emulate_cycle();
    cpu.display.draw_screen();
  }
}
