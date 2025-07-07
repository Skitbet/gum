use std::process::Command;

use iced::{ executor, Application };
use steamlocate::SteamDir;

use crate::{ app::GameLauncher, data::game::Game, platform::Platform };

mod data;
mod platform;
mod app;

fn main() -> iced::Result {
    GameLauncher::run(iced::Settings::default())
}
