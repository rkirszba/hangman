use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader, Write};

const DICO_FILE: &str = "./Dictionnaire";

/// Retourne un tableau avec les mots du dictionnaire.
/// Si le dictionnaire n'existe pas, retourne un tableau vide
pub fn load_words() -> Vec<String> {
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

/// Exporte les mots dans le fichier de dictionnaire
pub fn save_words(words: &[String]) -> Result<(), Box<dyn Error>> {
    let mut file = fs::OpenOptions::new()
        .write(true).create(true)
        .open(DICO_FILE)?;
    for word in words.iter() {
        writeln!(file, "{}", word)?;
    }
    Ok(())
}