use std::fs;
use std::cmp::Ordering;
use serde_json::{Value, json};

///
/// A more rusty-way to solve this one is to impl PartialOrd and Ord to use the comparison
/// operators directly:
///     https://fasterthanli.me/series/advent-of-code-2022/part-13
///

fn sign(x: i64) -> i64 {
    if x < 0 {
        -1
    } else if x > 0 {
        1
    } else {
        0
    }
}

/// Takes JSON value as input and compare them.
/// Returns
///     * -1 if a < b
///     * 1 if a > b
///     * 0 if equals
fn cmp_packets(a: &Value, b: &Value) -> i64 {
    // both are integers
    if a.is_number() && b.is_number() {
        return sign(a.as_i64().unwrap() - b.as_i64().unwrap());
    }

    // make a list from the single integer
    if a.is_array() && !b.is_array() {
        let new_b = match b.as_u64() {
            Some(v) => json![[v]],
            None    => json![[]]
        };
        return cmp_packets(a, &new_b);
    }
    if b.is_array() && !a.is_array() {
        let new_a = match a.as_u64() {
            Some(v) => json![[v]],
            None    => json![[]]
        };
        return cmp_packets(&new_a, b);
    }

    // both are lists so recurse
    let vec_a = a.as_array().unwrap();
    let vec_b = b.as_array().unwrap();
    for (sub_a, sub_b) in vec_a.iter().zip(vec_b.iter()){
        let res = cmp_packets(sub_a, sub_b);

        // return only if the pair differ, array length checked later
        if res != 0 {
            return res;
        }
    }

    // one of the two list fully contain the other one
    return sign(vec_a.len() as i64 - vec_b.len() as i64);
}

fn part1(lines: &Vec<&str>) -> u64 {
    let mut sum: u64 = 0;
    let mut pair_id = 0;

    for i in (0..lines.len()-1).step_by(2) {
        // parse 'left' and 'right'
        pair_id += 1;
        let va: Value = serde_json::from_str(lines[i]).unwrap();
        let vb: Value = serde_json::from_str(lines[i+1]).unwrap();

        // compare
        if cmp_packets(&va, &vb) < 0 {
            sum += pair_id;
        }
    }

    sum
}

fn part2(lines: &mut Vec<&str>) -> u64 {
    let mut dividers = vec!["[2]", "[6]"];
    let mut sum = 1;

    lines.append(&mut dividers);
    lines.sort_by(|a, b| {
        let va: Value = serde_json::from_str(a).unwrap();
        let vb: Value = serde_json::from_str(b).unwrap();
        match cmp_packets(&va, &vb) {
            -1 => Ordering::Less,
            0  => Ordering::Equal,
            1  => Ordering::Greater,
            _  => panic!("part2: WTF!?"),
        }
    });

    for (i, l) in lines.iter().enumerate() {
        if *l == "[2]" || *l == "[6]" {
            sum *= i+1;
        }
    }

    sum as u64
}

#[allow(dead_code)]
pub fn solution() {
    let input = fs::read_to_string("resources/13_input_00.txt")
        .expect("[-] Couldn't read the file");
    let mut lines: Vec<&str> = input.split("\n")
        .filter(|l| *l != "")
        .collect();

    let p1 = part1(&lines);
    let p2 = part2(&mut lines);

    println!("[13] Solution a: {}", p1);
    println!("[13] Solution b: {}", p2);
}
