extern crate sdl;

use std::io;
use sdl::event::Event;

use cpu::Cpu;

mod cpu;
mod display;
mod keypad;

fn main() {
    let mut cpu = Cpu::new();

    let input_value = io::stdin().read_line().unwrap();
    let game = format!("games/{}", input_value);

    cpu.load_game(game);
    sdl::init(&[sdl::InitFlag::Video, sdl::InitFlag::Audio, sdl::InitFlag::Timer]);

    'main : loop {
        'event : loop {
            match sdl::event::poll_event() {
                Event::Quit                  => break 'main,
                Event::None                  => break 'event,
                Event::Key(key, state, _, _) => cpu.keypad.press(key, state),
                _                                  => {}
            }
        }

        cpu.emulate_cycle();
        cpu.display.draw_screen();
    }

    sdl::quit();
}
