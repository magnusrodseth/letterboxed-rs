use clap::Parser;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn load_word_list(file_path: &str) -> Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut words = Vec::new();
    for line in reader.lines() {
        let word = line?;
        words.push(word);
    }
    Ok(words)
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The box of words, separated by commas
    /// @example "abc,def,ghi,jkl"
    #[arg(short, long)]
    grid: String,
}

fn is_valid_args_length(args: &Args) -> bool {
    args.grid.split(',').count() == 4
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug)]
struct Grid {
    words: HashMap<Side, Vec<char>>,
    dictionary: Vec<String>,
}

impl Grid {
    fn new(grid: String, dictionary: Vec<String>) -> Self {
        let sides = [Side::Top, Side::Right, Side::Bottom, Side::Left];
        let mut words = HashMap::new();
        for (side, word) in sides.iter().zip(grid.split(',')) {
            words.insert(*side, word.chars().collect());
        }
        Self { words, dictionary }
    }

    fn is_valid(&self) -> bool {
        self.words.len() == 4 && self.words.values().all(|word| word.len() == 3)
    }

    fn generate_words(&self) -> Vec<String> {
        let mut valid_words = Vec::new();
        for word in &self.dictionary {
            if self.is_valid_word(word) {
                valid_words.push(word.clone());
            }
        }
        valid_words
    }

    /// Words must be at least 3 letters long
    /// Consecutive letters cannot be from the same side
    fn is_valid_word(&self, word: &str) -> bool {
        if word.len() < 3 {
            return false;
        }

        let mut last_side = None;
        for letter in word.chars() {
            if let Some(side) = self.get_side(&letter) {
                if Some(side) == last_side {
                    return false;
                }
                last_side = Some(side);
            } else {
                return false; // Letter not found in any side
            }
        }
        true
    }

    fn get_side(&self, letter: &char) -> Option<Side> {
        for (side, letters) in &self.words {
            if letters.contains(letter) {
                return Some(*side);
            }
        }
        None
    }
}

fn main() {
    let args = Args::parse();
    let dictionary = load_word_list("words.txt").expect("Invalid file path.");
    let game = Grid::new(args.grid, dictionary);

    if !game.is_valid() {
        println!("Invalid grid formation.");
        return;
    }

    let generated_words = game.generate_words();
    println!("Generated words: {:?}", generated_words);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY_DICTIONARY: Vec<String> = vec![];

    impl Grid {
        fn new_with_dictionary(grid: String, dictionary: Vec<String>) -> Self {
            let sides = [Side::Top, Side::Right, Side::Bottom, Side::Left];
            let mut words = HashMap::new();
            for (side, word) in sides.iter().zip(grid.split(',')) {
                words.insert(*side, word.chars().collect());
            }
            Self { words, dictionary }
        }
    }

    #[test]
    fn test_grid_is_valid() {
        let grid = Grid::new("abc,def,ghi,jkl".to_string(), EMPTY_DICTIONARY);
        assert_eq!(grid.is_valid(), true);
    }

    #[test]
    fn test_is_valid_args_length() {
        let args = Args {
            grid: "abc,def,ghi,jkl".to_string(),
        };
        assert_eq!(is_valid_args_length(&args), true);
    }

    #[test]
    fn test_is_invalid_args_length() {
        let args = Args {
            grid: "abc,def,ghi".to_string(),
        };
        assert_eq!(is_valid_args_length(&args), false);
    }

    #[test]
    fn test_grid_has_too_few_letters() {
        let grid = Grid::new("ab,def,ghi,jkl".to_string(), EMPTY_DICTIONARY);
        assert_eq!(grid.is_valid(), false);
    }

    #[test]
    fn test_grid_has_too_many_letters() {
        let grid = Grid::new("abcd,def,ghi,jkl".to_string(), EMPTY_DICTIONARY);
        assert_eq!(grid.is_valid(), false);
    }

    #[test]
    fn test_is_valid_word_valid() {
        let grid = Grid::new("abc,def,ghi,jkl".to_string(), EMPTY_DICTIONARY);
        assert!(grid.is_valid_word("beg"));
    }

    #[test]
    fn test_is_valid_word_invalid_single_side() {
        let grid = Grid::new("abc,def,ghi,jkl".to_string(), EMPTY_DICTIONARY);
        assert!(!grid.is_valid_word("ace"));
    }

    #[test]
    fn test_is_valid_word_invalid_nonexistent_letter() {
        let grid = Grid::new("abc,def,ghi,jkl".to_string(), EMPTY_DICTIONARY);
        assert!(!grid.is_valid_word("xyz"));
    }

    #[test]
    fn test_generate_words() {
        let dictionary = vec![
            "beg".to_string(),
            "ace".to_string(),
            "xyz".to_string(),
            "fij".to_string(),
        ];
        let grid = Grid::new_with_dictionary("abc,def,ghi,jkl".to_string(), dictionary.clone());
        let generated_words = grid.generate_words();
        let expected_words: Vec<String> = vec!["beg".to_string(), "fij".to_string()];

        assert_eq!(generated_words, expected_words);
    }
}
