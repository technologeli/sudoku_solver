# Sudoku Solver
After working on my [Map Colorizer](https://technologeli.github.io/map-colorizer)
I wanted to utilize the backtracking algorithm to solve Sudoku puzzles.
I also learned a little bit of Rust along the way.

## Installation
To build and run the app, clone the repository and execute the follwing command.
```
cargo run
```

Use `CTRL-C` to close the program.

## Puzzles
The puzzles are held in the `puzzles/` directory and are text files named `#.txt`
where `#` is a number. The text files are 9 rows of 9 digits with 0 representing
a blank square in Sudoku. Feel free to add your own puzzles to solve.
