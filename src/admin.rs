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
