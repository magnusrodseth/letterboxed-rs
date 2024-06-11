use clap::Parser;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const DEFAULT_MAX_GUESSES: usize = 6;

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
    /// The box of words, separated by commas. An example box would be "abc,def,ghi,jkl".
    #[arg(short, long)]
    grid: String,

    /// The maximum number of guesses to make
    #[arg(short, long)]
    max_guesses: Option<usize>,
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
    all_letters: HashSet<char>,
    max_guesses: usize,
}

impl Grid {
    fn new(grid: String, dictionary: Vec<String>, max_guesses: Option<usize>) -> Self {
        let sides = [Side::Top, Side::Right, Side::Bottom, Side::Left];
        let mut words = HashMap::new();

        let mut all_letters = HashSet::new();
        for (side, word) in sides.iter().zip(grid.split(',')) {
            let chars: Vec<char> = word.chars().collect();
            all_letters.extend(&chars);
            words.insert(*side, chars);
        }

        Self {
            words,
            dictionary,
            all_letters,
            max_guesses: max_guesses.unwrap_or(DEFAULT_MAX_GUESSES),
        }
    }

    fn is_valid(&self) -> bool {
        self.words.len() == 4
            && self.words.values().all(|word| word.len() == 3)
            && self.all_letters.len() == 12
    }

    fn generate_words(&self) -> Vec<String> {
        self.dictionary
            .iter()
            .filter(|&&ref word| self.is_valid_word(word))
            .cloned()
            .collect()
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

    fn solve(&self) -> Option<Vec<String>> {
        let valid_words = self.generate_words();

        let solution = self.solve_bfs(&valid_words);
        if let Some(solution) = solution {
            if self.is_solution_valid(&solution) {
                return Some(solution);
            }
        }

        None
    }

    fn solve_bfs(&self, valid_words: &[String]) -> Option<Vec<String>> {
        let mut heap = BinaryHeap::new();

        for word in valid_words {
            let mut used_letters = HashSet::new();
            for ch in word.chars() {
                used_letters.insert(ch);
            }
            let mut used_letters_vec: Vec<char> = used_letters.iter().copied().collect();
            used_letters_vec.sort_unstable();
            heap.push(Reverse((1, used_letters_vec, vec![word.clone()])));
        }

        while let Some(Reverse((count, used_letters_vec, path))) = heap.pop() {
            let used_letters: HashSet<char> = used_letters_vec.iter().copied().collect();
            if used_letters.len() == self.all_letters.len() {
                return Some(path);
            }

            if count >= self.max_guesses {
                continue;
            }

            for word in valid_words {
                if word.chars().next().unwrap() == path.last().unwrap().chars().last().unwrap()
                    && !path.contains(word)
                {
                    let mut new_used_letters = used_letters.clone();
                    for ch in word.chars() {
                        new_used_letters.insert(ch);
                    }
                    let mut new_used_letters_vec: Vec<char> =
                        new_used_letters.iter().copied().collect();
                    new_used_letters_vec.sort_unstable();
                    let mut new_path = path.clone();
                    new_path.push(word.clone());
                    heap.push(Reverse((count + 1, new_used_letters_vec, new_path)));
                }
            }
        }

        None
    }

    fn is_solution_valid(&self, solution: &[String]) -> bool {
        let mut used_letters = HashSet::new();
        for word in solution {
            for ch in word.chars() {
                used_letters.insert(ch);
            }
        }
        used_letters == self.all_letters
    }
}

fn main() {
    let args = Args::parse();

    if !is_valid_args_length(&args) {
        println!("Invalid grid formation. Use `--help` to see the correct format.");
        return;
    }

    let dictionary = load_word_list("words.txt").expect("Invalid file path.");
    let game = Grid::new(args.grid.to_uppercase(), dictionary, args.max_guesses);

    if !game.is_valid() {
        println!("Invalid grid formation. Use `--help` to see the correct format.");
        return;
    }

    match game.solve() {
        Some(solution) => println!("Solution found: {:?}", solution),
        None => println!("No solution found."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY_DICTIONARY: Vec<String> = vec![];

    #[test]
    fn test_grid_is_valid() {
        let grid = Grid::new("abc,def,ghi,jkl".to_string(), EMPTY_DICTIONARY, None);
        assert_eq!(grid.is_valid(), true);
    }

    #[test]
    fn test_is_valid_args_length() {
        let args = Args {
            grid: "abc,def,ghi,jkl".to_string(),
            max_guesses: None,
        };
        assert_eq!(is_valid_args_length(&args), true);
    }

    #[test]
    fn test_is_invalid_args_length() {
        let args = Args {
            grid: "abc,def,ghi".to_string(),
            max_guesses: None,
        };
        assert_eq!(is_valid_args_length(&args), false);
    }

    #[test]
    fn test_grid_has_too_few_letters() {
        let grid = Grid::new("ab,def,ghi,jkl".to_string(), EMPTY_DICTIONARY, None);
        assert_eq!(grid.is_valid(), false);
    }

    #[test]
    fn test_grid_has_too_many_letters() {
        let grid = Grid::new("abcd,def,ghi,jkl".to_string(), EMPTY_DICTIONARY, None);
        assert_eq!(grid.is_valid(), false);
    }

    #[test]
    fn test_is_valid_word_valid() {
        let grid = Grid::new("abc,def,ghi,jkl".to_string(), EMPTY_DICTIONARY, None);
        assert!(grid.is_valid_word("beg"));
    }

    #[test]
    fn test_is_valid_word_invalid_single_side() {
        let grid = Grid::new("abc,def,ghi,jkl".to_string(), EMPTY_DICTIONARY, None);
        assert!(!grid.is_valid_word("ace"));
    }

    #[test]
    fn test_is_valid_word_invalid_nonexistent_letter() {
        let grid = Grid::new("abc,def,ghi,jkl".to_string(), EMPTY_DICTIONARY, None);
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
        let grid = Grid::new("abc,def,ghi,jkl".to_string(), dictionary.clone(), None);
        let generated_words = grid.generate_words();
        let expected_words: Vec<String> = vec!["beg".to_string(), "fij".to_string()];

        assert_eq!(generated_words, expected_words);
    }
}
