use std::{fs,cmp};
use pathfinding::prelude::dijkstra;

// up, down, left, right
const DIRECTIONS: [(i64, i64); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

/// Returns a vector of neighboring position matching the constraints and the high associated with
/// each of them.
fn get_neighbors(map: &Vec<Vec<u32>>, pos: (usize, usize)) -> Vec<((usize, usize), u32)> {
    let mut neighbors: Vec<((usize, usize), u32)> = Vec::new();
    let max_high = map[pos.0][pos.1] + 1;

    for dir in DIRECTIONS {
        let x = pos.0 as i64 + dir.0;
        let y = pos.1 as i64 + dir.1;

        // check map boundaries
        if x < 0 || x >= map.len() as i64 || y < 0 || y >= map[0].len() as i64 {
            continue;
        }

        let nh = map[x as usize][y as usize] as u32;
        if nh <= max_high {
            neighbors.push(((x as usize, y as usize), nh));
        }
    }

    neighbors
}

fn init() -> Vec<Vec<u32>> {
    //let map: Vec<Vec<u32>> = fs::read_to_string("resources/test.txt")
    let map: Vec<Vec<u32>> = fs::read_to_string("resources/12_input_00.txt")
        .expect("[-] Couldn't read the file")
        .split("\n")
        .filter(|l| *l != "")
        .map(|l| l.chars()
            .map(|c| c as u32)
            .collect())
        .collect();

    map
}

fn part1(map: &mut Vec<Vec<u32>>) -> u64 {
    let mut start = (0,0);
    let mut target = (0,0);
    let rows = map.len();
    let cols = map[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if map[row][col] == 'S' as u32 {
                map[row][col] = 'a' as u32;
                start = (row, col);
            }

            if map[row][col] == 'E' as u32 {
                map[row][col] = 'z' as u32;
                target = (row, col);
            }
        }
    }

    let result = dijkstra(&start, |p| get_neighbors(&map, *p), |p| *p == target);
    let steps = result.unwrap().0;

    steps.len() as u64 - 1
}

fn part2(map: &mut Vec<Vec<u32>>) -> u64 {
    let mut starts: Vec<(usize, usize)> = Vec::new();
    let mut target = (0,0);
    let mut shortest: u64 = u64::MAX;
    let rows = map.len();
    let cols = map[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if map[row][col] == 'S' as u32 || map[row][col] == 'a' as u32 {
                map[row][col] = 'a' as u32;
                starts.push((row, col));
            }

            if map[row][col] == 'E' as u32 {
                map[row][col] = 'z' as u32;
                target = (row, col);
            }
        }
    }

    for i in 0..starts.len() {
        let result = dijkstra(&starts[i], |p| get_neighbors(&map, *p), |p| *p == target);
        let steps = match result {
            Some(steps) => steps.0.len() as u64,
            None => u64::MAX,
        };

        shortest = cmp::min(shortest, steps);
    }

    shortest as u64 - 1
}

#[allow(dead_code)]
pub fn solution() {
    let mut map = init();
    let p1 = part1(&mut map);
    println!("[12] solution a: {}", p1);

    let mut map = init();
    let p2 = part2(&mut map);
    println!("[12] solution b: {}", p2);
}
