use cpu::Cpu;

pub mod cpu;

fn main() {
  let cpu = Cpu::new();
  cpu.emulate_cycle();
  println(fmt!("%?", (0xFF << 8 | 0xff)));
}
