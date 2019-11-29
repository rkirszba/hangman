use std::fs;
use std::io::{BufRead, BufReader};

pub const DICO_FILE: &str = "./Dictionnaire";

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