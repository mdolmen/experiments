use std::{fs, cmp};

#[derive(Debug, Clone, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug, PartialEq)]
enum Cave {
    Air,
    Rock,
    Sand,
}

impl Point {
    fn parse(s: &str) -> Self {
        let coord: Vec<&str> = s.split(",").collect();

        if coord.len() != 2 {
            return Self { x:0, y:0 };
        }
        
        Self {
            x: coord[0].parse().unwrap(),
            y: coord[1].parse().unwrap(),
        }
    }
}

fn init() -> (Vec<Vec<Cave>>, usize) {
    //let input = fs::read_to_string("resources/test.txt")
    //    .expect("[-] Couldn't read the file");
    let input = fs::read_to_string("resources/14_input_00.txt")
        .expect("[-] Couldn't read the file");
    let lines: Vec<&str> = input
        .split("\n")
        .filter(|l| *l != "")
        .collect();
    let mut max_y = 0;
    let mut grid: Vec<Vec<Cave>> = Vec::with_capacity(1000);

    for _ in 0..1000 {
        grid.push(vec![Cave::Air; 1000]);
    }

    for l in 0..lines.len() {
        let line = lines[l];
        let coords = line.split(" -> ");
        let mut previous = Point { x: 0, y: 0 };
        let mut is_first = true;

        for c in coords {
            if is_first {
                previous = Point::parse(c);
                max_y = cmp::max(previous.y, max_y);
                is_first = false;
                continue
            }

            let current = Point::parse(c);

            max_y = cmp::max(current.y, max_y);

            if current.y != previous.y {
                let start = cmp::min(current.y, previous.y);
                let end = cmp::max(current.y, previous.y);

                for y in start..=end {
                    grid[current.x][y] = Cave::Rock;
                }
            }

            else if current.x != previous.x {
                let start = cmp::min(current.x, previous.x);
                let end = cmp::max(current.x, previous.x);

                for x in start..=end {
                    grid[x][current.y] = Cave::Rock;
                }
            }

            previous = current;
        }
    }

    (grid, max_y)
}

fn drop_sand(grid: &mut Vec<Vec<Cave>>, max_y: usize) -> (usize, usize) {
    let mut total = 0;
    let mut total_p1 = 0;
    let mut p1_done = false;

    loop {
        let (mut x, mut y) = (500, 0);
        let mut stop_p1 = true;

        if grid[x][y] == Cave::Sand {
            break;
        }

        while y <= max_y {
            if grid[x][y+1] == Cave::Air {
                y += 1;
                continue;
            } else if grid[x-1][y+1] == Cave::Air {
                x -= 1;
                y += 1;
                continue;
            } else if grid[x+1][y+1] == Cave::Air {
                x += 1;
                y += 1;
                continue;
            }

            // Nothing moves anymore
            stop_p1 = false;
            break;
        }

        total += 1;
        grid[x][y] = Cave::Sand;

        if stop_p1 && !p1_done {
            total_p1 = total;
            p1_done = true;
        }
    }

    (total_p1, total)
}

pub fn solution() {
    let (mut grid, max_y) = init();
    let (p1, p2) = drop_sand(&mut grid, max_y);

    println!("[14] Solution a: {}", p1);
    println!("[14] Solution b: {}", p2);
}
