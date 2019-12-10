use std::env;
use std::error::Error;

use hangman::admin::Admin;
use hangman::config;
use hangman::game::Game;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mode = config::parse_args(args)?;
    match mode {
        config::Mode::Admin => Admin::new()?.run(),
        config::Mode::Game => Game::new()?.run()
    }
}
