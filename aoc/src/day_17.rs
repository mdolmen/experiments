/// 
/// Solution part 2:
///     https://www.ericburden.work/blog/2022/12/18/advent-of-code-2022-day-17/
///

use std::fs;
use std::collections::HashSet;

fn is_within_chamber(rock: &Vec<(i64, i64)>) -> bool {
    let mut outside = false;

    for (x, _) in rock.iter() {
        outside |= *x < 0i64 || *x >= 7i64;
    }

    !outside
}

fn is_solid(solid: &HashSet<(i64, i64)>, rock: &Vec<(i64, i64)>) -> bool {
    let mut ok = false;

    for spot in rock.iter() {
        if solid.contains(&spot) {
            ok = true;
        }
    }

    ok
}

pub fn solve() -> i64 {
    let file = fs::read_to_string("resources/17_input_00.txt")
        .expect("[-] Couldn't read the file"); let input = file
        .trim_end_matches('\n');

    let rocks: Vec<Vec<(i64, i64)>> = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    let jets: Vec<i64> = input.chars().map(|x| if x == '>' { 1 } else { -1 }).collect();
    let mut solid = (0..7).map(|x| (x, -1)).collect::<HashSet<(i64, i64)>>();
    let mut height = 0;
    let mut rock_count = 0;
    let target = 2022;
    let rocks_len = rocks.len();

    // Get rock template and add 2 for x axis and 3 for y axis to place it at its starting
    // position.
    let mut rock: Vec<(i64, i64)> = rocks[rock_count % rocks_len].clone()
        .into_iter()
        .map(|x| (x.0 + 2, height + 3))
        .collect();

    while rock_count < target {
        //let jet_index = 0;

        for jet in &jets {

            // Apply 'jet' force
            let mut moved = rock.clone()
                .into_iter()
                .map(|x| (x.0 + jet, x.1))
                .collect::<Vec<(i64, i64)>>();

            // check its boundaries
            if is_within_chamber(&moved) && !is_solid(&solid, &moved) {
                //println!("moved is inside chamber = {:?}", moved);
                rock = moved;
            }

            // drop it one level
            moved = rock.clone()
                .into_iter()
                .map(|x| (x.0, x.1 - 1))
                .collect::<Vec<(i64, i64)>>();

            // check again boudaries
            if is_solid(&solid, &moved) {
                solid.extend(rock.clone());

                let max = solid.iter().map(|&(_, y)| y).max().unwrap_or(0);
                height = max + 1;

                rock_count += 1;
                if rock_count >= target {
                    break;
                }
                let rock_index = rock_count % rocks_len;
                rock = rocks[rock_index].clone()
                    .into_iter()
                    .map(|x| (x.0 + 2, x.1 + height + 3))
                    .collect();
            } else {
                rock = moved.clone();
            }
        }
    }

    height
}

pub fn solution() {
    let p1 = solve();

    println!("[16] Solution a: {}", p1);
    //println!("[16] Solution b: {}", p2);
}
