use std::{fs, cmp};

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
    facing: u64,
}

#[derive(Debug, Clone, Default)]
struct Tile {
    x: usize,
    y: usize,
    is_wall: bool,
    is_open: bool,
}

const RIGHT: u64 = 0;
const DOWN: u64 = 1;
const LEFT: u64 = 2;
const UP: u64 = 3;

impl Pos {
    fn update(&mut self, board: &Vec<Vec<Tile>>, n: usize, dir: &String) {
        let dir = dir.chars().nth(0).unwrap();
        let nb_row = board.len();
        let mut backup = (0, 0);

        'outer: for _i in 0..n {
            // move until we're done or not possible given the board setting
            loop {
                if board[self.x][self.y].is_open {
                    backup = (self.x, self.y);
                }

                match self.facing {
                    RIGHT => {
                        self.y = (self.y+1) % board[self.x].len();
                    },
                    DOWN => {
                        self.x = (self.x+1) % nb_row;
                    },
                    LEFT => {
                        self.y = self.y.checked_sub(1).unwrap_or(board[self.x].len()-1);
                    },
                    UP => {
                        self.x = self.x.checked_sub(1).unwrap_or(nb_row-1);
                    },
                    _   => panic!("update: WTF!?"),
                };
                //println!("({},{}): is_open = {}, is_wall = {}",
                //    self.x, self.y,
                //    board[self.x][self.y].is_open, board[self.x][self.y].is_wall
                //);

                // move through the spaces until the next actual tile of the board
                if board[self.x][self.y].is_open {
                    break;
                }

                if board[self.x][self.y].is_wall {
                    (self.x, self.y) = backup;
                    break 'outer;
                }
            }
        }

        // update facing
        match dir {
            'R' => { self.facing = (self.facing + 1) % 4; },
            'L' => { self.facing = self.facing.checked_sub(1).unwrap_or(3); },
            _   => (),
        }
    }
}

/// Returns the 'board' and the 'instructions' to executes.
fn init() -> (Vec<Vec<Tile>>, Pos, Vec<String>) {
    //let input = fs::read_to_string("resources/test.txt")
    //    .expect("[-] Couldn't read the file");
    let input = fs::read_to_string("resources/22_input_00.txt")
        .expect("[-] Couldn't read the file");
    let split: Vec<&str> = input.split("\n\n").filter(|l| *l != "").collect();
    let map: Vec<Vec<char>> = split[0].split("\n")
        .map(|l| l.chars().collect())
        .collect();
    let nb_row = map.len();
    let mut nb_col_max = map[0].len();
    let mut board: Vec<Vec<Tile>> = vec![vec![]; nb_row];
    let mut pos = Pos { x: 0, y: 0, facing: RIGHT};
    let mut start_seen = false;
    let mut instructions: Vec<String> = Vec::new();

    // create the board
    for row in 0..nb_row {
        let this_cols = map[row].len();
        nb_col_max = cmp::max(nb_col_max, this_cols);

        for col in 0..this_cols {
            let is_wall = map[row][col] == '#';
            let is_open = map[row][col] == '.';
            board[row].push(Tile { x: row, y: col, is_wall: is_wall, is_open: is_open});

            if is_open && !start_seen {
                pos.x = row;
                pos.y = col;
                start_seen = true;
            }
        }

        // fill the void to make it an even matrix
        for col in this_cols..nb_col_max {
            board[row].push(Tile { x: row, y: col, is_wall: false, is_open: false});
        }
    }

    // create the instructions
    let mut num: Vec<char> = Vec::new();
    for c in split[1].chars() {
        if c.is_alphabetic() {
            instructions.push(num.iter().collect::<String>());
            instructions.push(c.to_string());
            num.clear();
        } else {
            num.push(c);
        }
    }

    (board, pos, instructions)
}

fn solve(board: &Vec<Vec<Tile>>, pos: &mut Pos, instructions: &Vec<String>) -> u64 {
    for mut i in 0..instructions.len() {
        match instructions[i].parse::<usize>() {
            Ok(n) => {
                i += 1;
                pos.update(board, n, &instructions[i])
            },
            Err(_) => ()
        }
    }

    1000 * (pos.x+1) as u64 + 4 * (pos.y+1) as u64 + pos.facing
}

#[allow(dead_code)]
pub fn solution() {
    let (board, mut pos, instructions) = init();
    let p1 = solve(&board, &mut pos, &instructions);

    println!("[22] solution a: {}", p1);
    //println!("[22] solution b: {}", p2);
}
