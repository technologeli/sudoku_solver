use std::fs;
use std::io;

struct Sudoku {
    board: [[u8; 9]; 9],
}

impl Sudoku {
    fn new(filename: &str) -> Sudoku {
        let contents = fs::read_to_string(filename).expect("Unable to read the file.");

        let mut s = Sudoku { board: [[0; 9]; 9] };

        for (i, line) in contents.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                let c = String::from(c);
                s.board[i][j] = c.parse().expect("Expected a number");
            }
        }

        s
    }

    fn is_possible(&self, y: usize, x: usize, n: u8) -> bool {
        for i in 0..9 {
            // check row
            if self.board[y][i] == n {
                return false;
            }

            // check col
            if self.board[i][x] == n {
                return false;
            }
        }

        // determine 3x3 grid using integer division
        let gy = y / 3 * 3;
        let gx = x / 3 * 3;

        /*
          for example, if we were checking for the middle at self.board[4][4],
          gy would be 3 and gx would be 3
        */

        // check 3x3 grid
        for i in 0..3 {
            for j in 0..3 {
                if self.board[gy + i][gx + j] == n {
                    return false;
                }
            }
        }

        // passes all checks
        return true;
    }

    fn print(&self) {
        for row in self.board {
            for cell in row {
                print!("{} ", cell);
            }
            println!();
        }
    }

    fn solve(&mut self) {
        for y in 0..9 {
            for x in 0..9 {
                if self.board[y][x] == 0 {
                    for n in 1..10 {
                        if self.is_possible(y, x, n) {
                            self.board[y][x] = n;
                            self.solve();
                            self.board[y][x] = 0;
                        }
                    }
                    return;
                }
            }
        }
        println!("Solution:");
        self.print();
        println!();
    }
}

fn display_puzzles() {
    let mut paths: Vec<_> = fs::read_dir("puzzles")
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    paths.sort_by_key(|dir| dir.path());

    for (i, path) in paths.iter().enumerate() {
        println!("Puzzle {}:", i + 1);
        match path.path().to_str() {
            Some(n) => display_puzzle(n),
            None => {}
        }
        println!();
    }
}

fn display_puzzle(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Unable to read the file.");

    for line in contents.lines() {
        for c in line.chars() {
            print!("{} ", c);
        }
        println!();
    }
}

fn main() {
    /* let mut s = Sudoku {
        board: [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ]
    }; */
    println!("Available puzzles:\n");
    display_puzzles();

    loop {
        let mut choice = String::new();

        println!("Choose a puzzle to solve:");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        println!();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut s = Sudoku::new(&format!("puzzles/{}.txt", choice));
        s.solve();
    }
}
