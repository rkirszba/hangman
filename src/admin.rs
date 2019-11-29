use std::error::Error;
use std::io;
use std::io::Write;

use crate::dico;
use crate::dico::DicoError;

pub struct Admin {
    words: Vec<String>
}

impl Admin {
    pub fn new() -> Result<Self, DicoError> {
        let words = dico::load_words()?;
        Ok(Admin { words })
    }

    fn display_words(&self) {
        if self.words.is_empty() {
            println!("\nIl n'y a pour l'instant aucun mot dans le dictionnaire\n");
        } else {
            println!("\nVoici les mots presents dans le dictionnaire");
            for word in self.words.iter() {
                println!("{}", word);
            }
        }
        println!();
    }

    fn add(&mut self, word: &str) -> Result<(), char> {
        let new_word = word.to_string().trim().to_uppercase();
        for c in new_word.chars() {
            if !c.is_alphabetic() {
                return Err(c);
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
        } else {
            println!("\n\"{}\" ne fait pas partie du dictionnaire\n", input.trim());
        }
        Ok(())
    }

    fn save(&mut self) -> Result<(), Box<dyn Error>> {
        self.words.sort();
        self.words.dedup();
        dico::save_words(&self.words)
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Bienvenue dans l'espace administrateur\n");
        loop {
            print!("Que voulez-vous faire ?\n\
                    d: afficher les mots\n\
                    a: ajouter un mot\n\
                    r: enlever un mot\n\
                    q: sauver et quitter\n\n\
                    C'est à vous: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let rq = input.trim().to_string().to_lowercase();
            match &rq[..] {
                "d" => self.display_words(),
                "a" => self.add_process()?,
                "r" => self.remove_process()?,
                "q" => {
                    self.save()?;
                    return Ok(());
                }
                _ => println!("\"{}\" n'est pas une option correcte\n", rq)
            }
        }
    }
}
