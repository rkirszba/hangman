use std::error::Error;
use std::fmt;
use std::io;
use std::io::Write;

use rand::seq::SliceRandom;

use crate::dico;

pub struct Game {
    word: String,
    matched_letters: String,
    unmatched_letters: String,
    rem_errors: u32,
}

impl Game {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        if let Some(word) = dico::load_words().choose(&mut rand::thread_rng()) {
            Ok(Game {
                word: word.into(),
                matched_letters: String::new(),
                unmatched_letters: String::new(),
                rem_errors: 6,
            })
        } else {
            Err(Box::new(DicoError("Le dictionnaire est vide !".into())))
        }
    }

    fn display_word(&self) {
        for (i, c) in self.word.chars().enumerate() {
            if i > 0 { print!(" "); }
            print!("{}", if self.matched_letters.contains(c) { c } else { '_' });
        }
        println!("\n");
    }

    fn ask_letter() -> Result<char, Box<dyn Error>> {
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let chars: Vec<char> = input.trim().chars().collect();
        match chars[..] {
            [c] if c.is_alphabetic() => {
                Ok(c.to_uppercase().next().unwrap_or(c))
            },
            _ => {
                println!("\"{}\" n'est pas une proposition correcte. \
                Veuillez réessayer.\n", input.trim());
                print!("Entrez une lettre: ");
                Game::ask_letter()
            }
        }
    }

    fn check_letter(&mut self, letter: char) -> bool {
        if self.word.contains(letter) {
            self.matched_letters.push(letter);
            true
        } else {
            self.unmatched_letters.push(letter);
            self.rem_errors -= 1;
            false
        }
    }

    fn display_errors(&self) {
        for (i, c) in self.unmatched_letters.chars().enumerate() {
            print!("{separator}{letter}",
                   separator = if i == 0 { "" } else { ", " },
                   letter = c);
        }
        println!("\n");
    }

    fn check_success(&self) -> bool {
        self.word.chars().all(|c| self.matched_letters.contains(c))
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Bienvenue dans le jeu du pendu !\n");
        loop {
            println!("Mot a trouver : ");
            self.display_word();
            print!("Entrez une lettre: ");
            let letter = Game::ask_letter()?;
            if self.check_letter(letter) { println!("'{}' fait bien partie du mot\n", letter); } else { println!("'{}' ne fait pas partie du mot\n", letter) };
            if self.check_success() {
                self.display_word();
                println!("Vous avez gagne, le mot a trouver etait bien \"{}\" !", self.word);
                return Ok(());
            } else if self.rem_errors == 0 {
                self.display_word();
                println!("Vous avez perdu, le mot a trouver etait \"{}\" !", self.word);
                return Ok(());
            }
            println!("Il vous reste {} essai(s).\n", self.rem_errors);
            if self.rem_errors != 6 {
                println!("Lettres proposees ne faisant pas partie du mot :");
                self.display_errors();
            }
        }
    }
}

#[derive(Debug)]
pub struct DicoError(String);

impl fmt::Display for DicoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for DicoError {}
