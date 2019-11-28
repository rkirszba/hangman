use std::env;
use std::process;

use hangman::admin::Admin;
use hangman::config;
use hangman::game::Game;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mode = config::parse_args(args)?;
    
    match mode {
        config::Mode::Admin => Admin::new()?.run(),
        config::Mode::Game => Game::new()?.run()
    }
}
