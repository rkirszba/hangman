use std::fs;
use std::io::{BufRead, BufReader};

const DICO_FILE: &str = "./Dictionnaire";

/// Retourne un tableau avec les mots du dictionnaire.
/// Si le dictionnaire n'existe pas, retourne un tableau vide
fn load_words() -> Vec<String> {
    if let Ok(dico) = fs::File::open(DICO_FILE) {
        BufReader::new(dico)
            .lines()
            .flatten()
            .map(|s| s.trim().to_uppercase().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        vec![]
    }
}

pub mod game {
    use std::error::Error;
    use std::fmt;
    use std::io;
    use std::io::Write;

    use rand::seq::SliceRandom;

    use crate::load_words;

    pub struct Game {
        word: String,
        matched_letters: String,
        unmatched_letters: String,
        rem_errors: u32,
    }

    impl Game {

        pub fn new() -> Result<Self, Box<dyn Error>> {
            if let Some(word) = load_words().choose(&mut rand::thread_rng()) {
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
                if self.check_letter(letter) { println!("'{}' fait bien partie du mot\n", letter); }
                else { println!("'{}' ne fait pas partie du mot\n", letter) };
                if self.check_success() {
                    self.display_word();
                    println!("Vous avez gagne, le mot a trouver etait bien \"{}\" !", self.word);
                    return Ok(());
                }
                else if self.rem_errors == 0 {
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
}

pub mod config {

    pub enum Mode {
        Admin,
        Game
    }

    pub fn parse_args(args: Vec<String>) -> Result<Mode, &'static str>
    {
        if args.len() > 2 { return Err("Usage: cargo run [admin]"); }
        if args.len() == 2 {
            let arg = args[1].as_str();
            match arg {
                arg if arg != "admin" => return Err("Usage: cargo run [admin]"),
                _ => return Ok(Mode::Admin)
            }
        }
        Ok(Mode::Game)
    }
}

pub mod admin {
    use std::error::Error;
    use std::fs;
    use std::io;
    use std::io::Write;

    use crate::{DICO_FILE, load_words};

    pub struct Admin {
        words: Vec<String>
    }

    impl Admin {
        pub fn new() -> Result<Self, Box<dyn Error>> {
            let words = load_words();
            Ok(Admin { words })
        }

        fn add(&mut self, word: &str) -> Result<(), char> {
            let new_word = word.to_string().trim().to_uppercase();
            for c in new_word.chars() {
                if !c.is_alphabetic() {
                    return Err(c)
                }
            }
            self.words.push(new_word.to_string());
            Ok(())
        }

        fn add_process(&mut self) -> Result<(), Box<dyn Error>> {
            println!("\nEntrez le mot que vous souhaitez ajouter: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if let Err(c) = self.add(&input) {
                println!("\n\"{word}\" n'est pas un mot valide car il contient le caractère '{invalid}'\n",
                         word = input.trim(),
                         invalid = c);
            } else {
                println!("\n\"{}\" a bien été ajouté au dictionnaire\n", input.trim());
            }
            Ok(())
        }

        fn remove(&mut self, word: &str) -> bool {
            let to_rem = word.to_string().trim().to_uppercase();
            for (i, entry) in self.words.iter().enumerate() {
                if entry == &to_rem {
                    self.words.remove(i);
                    return true;
                }
            }
            false
        }

        fn remove_process(&mut self) -> Result<(), Box<dyn Error>> {
            println!("\nEntrez le mot que vous souhaitez supprimer: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if self.remove(&input) {
                println!("\n\"{}\" a bien ete retire du dictionnaire\n", input.trim());
            }
            else {
                println!("\n\"{}\" ne fait pas partie du dictionnaire\n", input.trim());
            }
            Ok(())
        }

        fn save(&mut self) -> Result<(), Box<dyn Error>> {
            self.words.sort();
            self.words.dedup();
            let mut file = fs::OpenOptions::new()
                .write(true).create(true)
                .open(DICO_FILE)?;
            for word in self.words.iter() {
                writeln!(file, "{}", word)?;
            }
            Ok(())
        }

        pub fn run(&mut self) -> Result <(), Box<dyn Error>> {
            println!("Bienvenue dans l'espace administrateur\n");
            loop {
                print!("Que voulez-vous faire ?\n\
                        a: ajouter un mot\n\
                        r: enlever un mot\n\
                        q: sauver et quitter\n\n\
                        C'est à vous: ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let rq = input.trim().to_string().to_lowercase();
                match &rq[..] {
                    "a" => self.add_process()?,
                    "r" => self.remove_process()?,
                    "q" => { self.save()?; return Ok(()) },
                    _ => println!("\"{}\" n'est pas une option correcte\n", rq)
                }
            }
        }
    }
}
