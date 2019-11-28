use std::env;
use std::process;
use hangman::game::Game;
use hangman::admin::Admin;
use hangman::config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = config::parse_args(args).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1) });
    
    match mode {
        config::Mode::Admin => run_admin(),
        config::Mode::Game => run_game()
    }
}

fn run_game() {
    let game = Game::new();
    match game {
        Err(e) => { eprintln!("Error: {}", e); process::exit(1); },
        _ => match game.unwrap().run() {
            Err(e) => { eprintln!("Error: {}", e); process::exit(1); },
            _ => process::exit(0)
        }
    }
}

fn run_admin() {
    let admin = Admin::new();
    match admin {
        Err(e) => { eprintln!("Error: {}", e); process::exit(1); },
        _ => match admin.unwrap().run() {
            Err(e) => { eprintln!("Error: {}", e); process::exit(1); },
            _ => process::exit(0)
        }
    }
}
