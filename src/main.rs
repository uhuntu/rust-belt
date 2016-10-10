extern crate piston_window;
extern crate find_folder;
extern crate music;

mod color;
mod game;
mod menu;
mod settings;
mod story;

use piston_window::{ PistonWindow, WindowSettings};

const GAME_TITLE: &'static str = "Rust Belt";
const GAME_WINDOW_WIDTH: u32 = 1024;
const GAME_WINDOW_HEIGHT: u32 = 768;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {
    Menu,
    Action,
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new(GAME_TITLE,
                                                       [GAME_WINDOW_WIDTH, GAME_WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|error| { panic!("Failed to build PistonWindow: {}", error) });

    music::start::<Music, _>(|| {
        music::bind_file(Music::Menu, "./assets/The Last Ranger.mp3");
        music::bind_file(Music::Action, "./assets/Into the Field.mp3");
        music::play(&Music::Menu, music::Repeat::Forever);

        menu::Menu::new().run(&mut window, GAME_TITLE, GAME_WINDOW_WIDTH);
    });
}