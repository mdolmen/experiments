use std::fs;
use std::collections::HashSet;

// up, down, left, right
const DIRECTIONS: [(i64, i64); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

fn sign(x: i64) -> i64 {
    if x < 0 {
        -1
    } else if x > 0 {
        1
    } else {
        0
    }
}

#[allow(dead_code)]
pub fn solution() {
    let input = fs::read_to_string("resources/09_input_00.txt")
        .expect("[-] Couldn't read the file");
    let lines: Vec<Vec<&str>> = input.split("\n")
        .filter(|l| *l != "")
        .map(|l| l.split(" ")
            .collect())
        .collect();
    let mut tail = (0, 0);
    let mut head = (0, 0);
    let mut visited_p1: HashSet<(i64, i64)> = HashSet::new();
    let mut visited_p2: HashSet<(i64, i64)> = HashSet::new();
    const ROPE_LENGTH: usize = 10;
    let mut rope: [(i64, i64); ROPE_LENGTH] = [(0, 0); ROPE_LENGTH];

    for i in 0..lines.len() {
        let dir = lines[i][0].chars().nth(0).unwrap();
        let dir = match dir as char {
            'U' => 0,
            'D' => 1,
            'L' => 2,
            'R' => 3,
            _   => panic!("WTF!?")
        };
        let steps: u64 = lines[i][1].parse().unwrap();

        for _ in 0..steps {
            rope[0].0 += DIRECTIONS[dir].0; // x
            rope[0].1 += DIRECTIONS[dir].1; // y

            // part 2
            for i in 0..ROPE_LENGTH-1  {
                (head.0, head.1) = rope[i];
                (tail.0, tail.1) = rope[i + 1];
                let dx: i64 = head.0 - tail.0;
                let dy: i64 = head.1 - tail.1;

                // Tricks for Euclidian distance formula: https://github.com/mebeim/aoc
                if dx.pow(2) + dy.pow(2) > 2 {
                    tail.0 += sign(dx);
                    tail.1 += sign(dy);
                    rope[i + 1] = (tail.0, tail.1);

                }
            }

            visited_p1.insert(rope[1]);
            visited_p2.insert(rope[ROPE_LENGTH-1]);
        }
    }

    println!("[09] solution a: {}", visited_p1.len());
    println!("[09] solution b: {}", visited_p2.len());
}
