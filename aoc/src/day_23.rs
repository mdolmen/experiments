use std::{fs, cmp};
use std::collections::{VecDeque, HashMap};

const NORTH: i64 = 0;
const SOUTH: i64 = 1;
const WEST: i64 = 2;
const EAST: i64 = 3;

// north, south, west, east
const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug, Clone, Default, Eq, Hash, PartialEq)]
struct Elf {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy, Default, Eq, Hash, PartialEq)]
struct Move {
    x: i64,
    y: i64,
}

/// Just the coordinates of the four edge elves
struct Grid {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Move {
    fn check_surroundings(
        &mut self, elves: &HashMap<Elf, Move>, elf: &Elf, priorities: &VecDeque<i64>
    ) -> i64
    {
        let mut choice: i64 = -1;

        for i in 0..priorities.len() {
            let dir = priorities.get(i).unwrap();

            // if there's absolutely no one around, don't do anything!
            if self.check_north(elves, elf)
                && self.check_south(elves, elf)
                && self.check_west(elves, elf)
                && self.check_east(elves, elf)
            {
                break;
            }

            // the destination if the elf has to move
            self.x = elf.x + DIRECTIONS[*dir as usize].0;
            self.y = elf.y + DIRECTIONS[*dir as usize].1;

            if *dir == NORTH && self.check_north(elves, elf) {
                choice = NORTH;
                break;
            } else if *dir == SOUTH && self.check_south(elves, elf) {
                choice = SOUTH;
                break;
            } else if *dir == WEST && self.check_west(elves, elf) {
                choice = WEST;
                break;
            } else if *dir == EAST && self.check_east(elves, elf) {
                choice = EAST;
                break;
            }
        }

        choice
    }

    fn check_north(&self, elves: &HashMap<Elf, Move>, elf: &Elf) -> bool {
        elves.get(&Elf {x: elf.x-1, y: elf.y-1 }).is_none()
            && elves.get(&Elf {x: elf.x-1, y: elf.y }).is_none()
            && elves.get(&Elf {x: elf.x-1, y: elf.y+1 }).is_none()
    }

    fn check_south(&self, elves: &HashMap<Elf, Move>, elf: &Elf) -> bool {
        elves.get(&Elf {x: elf.x+1, y: elf.y-1 }).is_none()
            && elves.get(&Elf {x: elf.x+1, y: elf.y }).is_none()
            && elves.get(&Elf {x: elf.x+1, y: elf.y+1 }).is_none()
    }

    fn check_west(&self, elves: &HashMap<Elf, Move>, elf: &Elf) -> bool {
        elves.get(&Elf {x: elf.x-1, y: elf.y-1 }).is_none()
            && elves.get(&Elf {x: elf.x, y: elf.y-1 }).is_none()
            && elves.get(&Elf {x: elf.x+1, y: elf.y-1 }).is_none()
    }

    fn check_east(&self, elves: &HashMap<Elf, Move>, elf: &Elf) -> bool {
        elves.get(&Elf {x: elf.x-1, y: elf.y+1 }).is_none()
            && elves.get(&Elf {x: elf.x, y: elf.y+1 }).is_none()
            && elves.get(&Elf {x: elf.x+1, y: elf.y+1 }).is_none()
    }
}

fn init() -> HashMap<Elf, Move> {
    //let input = fs::read_to_string("resources/test.txt")
    //    .expect("[-] Couldn't read the file");
    let input = fs::read_to_string("resources/23_input_00.txt")
        .expect("[-] Couldn't read the file");
    let lines: Vec<Vec<char>> = input.split("\n")
        .filter(|l| *l != "")
        .map(|l| l.chars().collect())
        .collect();
    let mut elves: HashMap<Elf, Move> = HashMap::new();
    let len_row = lines[0].len();

    // get elves location
    for i in 0..lines.len() {
        let row = &lines[i];

        for j in 0..len_row {
            if row[j] == '#' {
                elves.insert(
                    Elf { x: i as i64, y: j as i64 },
                    Move { x: 0, y: 0 },
                );
            }
        }
    }

    elves
}

fn solve(elves: &mut HashMap<Elf, Move>, rounds: usize) -> i64 {
    let mut priorities = VecDeque::from([NORTH, SOUTH, WEST, EAST]);

    for round in 0..rounds {
        let mut have_moved = false; // part 2
        let mut proposals: HashMap<Move, i64> = HashMap::new();

        // hack to make satisfy the borrow checker...
        let mut coords: Vec<Elf> = Vec::new();
        for e in elves.keys() {
            coords.push(Elf {x: e.x, y: e.y});
        }

        // propose moves (note: should be a fct)
        for elf in &coords {
            // ...not satisfied with all of this, but BC OK
            let tmp = elves.get_mut(&elf).unwrap();
            let mut move_to = Move {x: tmp.x, y:tmp.y};
            let can_move = move_to.check_surroundings(elves, &elf, &priorities);
            if can_move >= 0 {
                // update the 'elves' HashMap (where each elf wants to go)
                let tmp = elves.get_mut(&elf).unwrap();
                tmp.x = move_to.x;
                tmp.y = move_to.y;
            }

            match proposals.get_mut(&move_to) {
                Some(count) => *count += 1,
                None => { proposals.insert(move_to, 1); },
            }
        }

        // actually move (note: should be a fct)
        for coord in &coords {
            let mov = elves.remove(&coord).unwrap();
            let allowed = match proposals.get(&mov) {
                Some(count) => {
                    if *count == 1 {
                        true
                    } else {
                        false
                    }
                },
                None => false
            };

            if allowed {
                have_moved = true;
                elves.insert(
                    Elf { x: mov.x as i64, y: mov.y as i64 },
                    Move { x: 0, y: 0 },
                );
            } else {
                elves.insert(
                    Elf { x: coord.x as i64, y: coord.y as i64 },
                    Move { x: 0, y: 0 },
                );
            }
        }

        // shift priorities
        let dir = priorities.pop_front().unwrap();
        priorities.push_back(dir);

        if !have_moved {
            println!("[23] solution b: {}", round+1);
            break;
        }

        //dbg_print_grid(elves);
    }

    return count_empty_tiles(elves);
}

#[allow(dead_code)]
fn dbg_print_grid(elves: &HashMap<Elf, Move>) {
    println!("\n");
    for i in 0..72 {
        for j in 0..72 {
            match elves.get(&Elf{x:i, y:j}) {
                Some(_) => { print!("#"); },
                None => { print!("."); }
            }
        }
        println!("");
    }
    println!("\n\n");
}

fn count_empty_tiles(elves: &HashMap<Elf, Move>) -> i64 {
    let mut empty: i64 = 0;
    let mut grid = Grid {
        min_x: i64::MAX, max_x: i64::MIN,
        min_y: i64::MAX, max_y: i64::MIN
    };

    for (elf, _) in elves {
        //println!("coord = {:?}", coord);
        grid.min_x = cmp::min(grid.min_x, elf.x);
        grid.max_x = cmp::max(grid.max_x, elf.x);
        grid.min_y = cmp::min(grid.min_y, elf.y);
        grid.max_y = cmp::max(grid.max_y, elf.y);
    }

    for i in grid.min_x..=grid.max_x {
        for j in grid.min_y..=grid.max_y {
            match elves.get(&Elf {x:i, y:j}) {
                Some(_) => (),
                None => { empty += 1; }
            }
        }
    }

    //println!("{}", grid.min_x);
    //println!("{}", grid.max_x);
    //println!("{}", grid.min_y);
    //println!("{}", grid.max_y);

    empty
}

#[allow(dead_code)]
pub fn solution() {
    let mut elves = init();
    let p1 = solve(&mut elves, 10);
    //let _p2 = solve(&mut elves, 1000);

    println!("[23] solution a: {}", p1);
    //println!("[23] solution b: {}", p2);
}
