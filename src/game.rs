pub mod game {

    use std::io;
    use std::io::{Write};
    use std::fs;
    use rand::Rng;
    use std::error::Error;
    use std::fmt;

    pub struct Game {
        word: String,
        matched_letters: String,
        unmatched_letters: String,
        rem_errors: u32
    }

    impl Game {

        pub fn new() -> Result<Self, Box<dyn Error>> {
            let dico = fs::read_to_string("./Dictionnaire")?;
            let vec: Vec<&str> = dico.split(|c: char| c == '\n')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();
            if vec.is_empty() {
                return Err(Box::new(DicoError("Le dictionnaire est vide !".into())));
            }
            let index = rand::thread_rng().gen_range(0, vec.len()) as usize;
            Ok(Game {
                word: vec[index].to_string().to_ascii_uppercase(),
                matched_letters: String::new(),
                unmatched_letters: String::new(),
                rem_errors: 6
            })
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
            input = input.trim().to_string();
            if input.is_empty() || input.len() > 1 || !input.as_bytes()[0 as usize].is_ascii_alphabetic() {
                println!("\"{}\" n'est pas une proposition correcte. \
                         Veuillez reessayer.\n", input);
                print!("Entrez une lettre: ");
                return Game::ask_letter();
            }
            Ok((input.as_bytes()[0].to_ascii_uppercase()) as char) 
        }

        fn check_letter(&mut self, letter: char) -> bool {
            match letter {
                letter if self.word.contains(letter) => {
                    self.matched_letters.push(letter);
                    true
                }
                _ => {
                    self.unmatched_letters.push(letter);
                    self.rem_errors -= 1;
                    false
                }
            }
        }

        fn display_errors(&self) {
            for (i, c) in self.unmatched_letters.chars().enumerate() {
                print!("{}{}", if i > 0 { ", " } else { "" }, c);
            }
            println!("\n");
        }

        fn check_success(&self) -> bool {
            for c in self.word.chars() {
                if !self.matched_letters.contains(c) {
                    return false;
                }
            }
            true
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
