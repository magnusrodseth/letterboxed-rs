# `letterboxed-rs`, a solver for the puzzle game "Letter Boxed"

## What is it?

Letter Boxed is a popular puzzle game by the New York Times. Try it out [here](https://www.nytimes.com/puzzles/letter-boxed). This project is a solver for the game, written in Rust.

## How to use it?

```sh
# Installing the application
cargo install letterboxed-rs

# Running the application
letterboxed --grid "abc,def,ghi,jkl"

# Optionally, you can set the maximum number of guesses to be made
letterboxed --grid "abc,def,ghi,jkl" --max-guesses 10

# Help
letterboxed --help
```

You'll notice that the grid is represented as a comma-separated string of words. Each word is a sequence of characters, and the words are separated by commas. The grid is a square, so the number of characters in each word is the same.

Each group of words represents a side of the square grid. The first group of words represents the top side, the second group represents the right side, the third group represents the bottom side, and the fourth group represents the left side. The order really doesn't matter, as long as you're consistent.

## Developer information

Developed by [Magnus Rødseth](https://github.com/magnusrodseth).
