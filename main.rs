extern mod sdl;

use cpu::Cpu;
use std::io;

pub mod cpu;

fn main() {
  let mut cpu = Cpu::new();
  let game = io::stdin().read_line();
  cpu.load_game(~"games/" + game);
  sdl::init([sdl::InitVideo, sdl::InitAudio, sdl::InitTimer]);

  'main : loop {
    'event : loop {
      match sdl::event::poll_event() {
        sdl::event::QuitEvent              => break 'main,
        sdl::event::NoEvent                => break 'event,
        sdl::event::KeyEvent(key, state, _, _) => cpu.keypad.press(key, state),
        _                                  => {}
      }
    }

    cpu.emulate_cycle();
    cpu.display.draw_screen();
  }

  sdl::quit();
}
