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
    
    use std::io;
    use std::io::Write;
    use std::fs;
    use std::error::Error;

    pub struct Admin {
        words : Vec<String>
    }

    impl Admin {
            
        pub fn new() -> Result<Self, Box<dyn Error>> {
            let dico = fs::read_to_string("./Dictionnaire")?;
            let split: Vec<&str> = dico.split(|c: char| c == '\n').collect();
            let mut words: Vec<String> = Vec::with_capacity(split.len());
            for s in split.iter() {
                let word = s.trim().to_string().to_ascii_uppercase();
                if !word.is_empty() {
                    words.push(word);
                }
            }
            Ok(Admin {words})
        }

        fn display_words(&self) {
            if self.words.is_empty() {
                println!("\nIl n'y a pour l'instant aucun mot dans le dictionnaire\n");
            }
            else {
                println!("\nVoici les mots presents dans le dictionnaire");
                for word in self.words.iter() {
                    println!("{}", word);
                }
            }
            println!("");
        }

        fn add(&mut self, word: &str) -> bool {
            let new_word = word.to_string().trim().to_ascii_uppercase();
            for c in new_word.chars() {
                if !c.is_alphabetic() {
                    return false
                }
            }
            self.words.push(new_word.to_string());
            true
        }

        fn add_process(&mut self) -> Result<(), Box<dyn Error>> {
            println!("\nEntrez le mot que vous souhaitez ajouter: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if self.add(&input) {
                println!("\n\"{}\" a bien ete ajoute au dictionnaire\n", input.trim());
            }
            else {
                println!("\n\"{}\" n'est pas un mot valide\n", input.trim());
            }
            Ok(())
        }

        fn remove(&mut self, word: &str) -> bool {
            let to_rem = word.to_string().trim().to_ascii_uppercase();
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
                .write(true)
                .create(true)
                .open("./Dictionnaire")?;
            for entry in self.words.iter() {
                writeln!(file, "{}", entry)?;
            }
            Ok(())
        }

        pub fn run(&mut self) -> Result <(), Box<dyn Error>> {
            println!("Bienvenue dans l'espace administrateur\n");
            loop {
                print!("Que voulez vous faire ?\n\
                       d: afficher les mots\n\
                       a: ajouter un mot\n\
                       r: enlever un mot\n\
                       q: sauver et quitter\n\n\
                       C'est a vous: ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let rq = input.trim().to_string().to_ascii_lowercase();
                match &rq[..] {
                    "d" => self.display_words(),
                    "a" => self.add_process()?,
                    "r" => self.remove_process()?,
                    "q" => { self.save()?; return Ok(()) },
                    _ => println!("\"{}\" n'est pas une option correcte\n", rq)
                }
            }
        }
    }
}
