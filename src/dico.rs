use std::{fmt, fs};
use std::error::Error;
use std::io::{BufRead, BufReader, Write};

const DICO_FILE: &str = "./Dictionnaire";

/// Retourne un tableau avec les mots du dictionnaire.
/// Si le dictionnaire n'existe pas, retourne un tableau vide
pub fn load_words() -> Result<Vec<String>, DicoError> {
    fs::File::open(DICO_FILE).map(|dico| {
        BufReader::new(dico)
            .lines()
            .flatten()
            .map(|s| s.trim().to_uppercase().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }).map_err(|e| DicoError::IO(e))
}

/// Exporte les mots dans le fichier de dictionnaire
pub fn save_words(words: &[String]) -> Result<(), DicoError> {
    let mut file = fs::OpenOptions::new()
        .write(true).create(true)
        .open(DICO_FILE)?;
    for word in words.iter() {
        writeln!(file, "{}", word)?;
    }
    Ok(())
}

pub enum DicoError {
    Empty,
    IO(std::io::Error),
}

impl fmt::Display for DicoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DicoError::Empty =>
                write!(f, "Le dictionnaire est vide !"),
            DicoError::IO(e) =>
                write!(f, "Le fichier de dictionnaire ({file}) n'est pas accessible. ({source})",
                       file = DICO_FILE,
                       source = e),
        }
    }
}

impl fmt::Debug for DicoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DicoError: {}", self)
    }
}

impl From<std::io::Error> for DicoError {
    fn from(source: std::io::Error) -> Self {
        DicoError::IO(source)
    }
}

impl Error for DicoError {}
