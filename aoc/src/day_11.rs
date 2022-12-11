use std::fs;
use std::collections::VecDeque;

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    seen: u64,
    op: String,
    op_a: String,
    op_b: String,
    divisor: u64,
    dst_true: usize,
    dst_false: usize,
}

fn build_monkeys() -> Vec<Monkey> {
    let input = fs::read_to_string("resources/11_input_00.txt")
        .expect("[-] Couldn't read the file");
    let lines: Vec<&str> = input.split("\n").filter(|l| *l != "").collect();
    let mut monkeys = Vec::new();

    // build list of Monkeys
    for i in (0..lines.len()).step_by(6) {
        // items
        let line: Vec<&str> = lines[i+1].split(":").collect();
        let items: VecDeque<u64> = line[1].split(",")
            .filter(|e| *e != "")
            .map(|e| e.trim().parse().unwrap())
            .collect();
        let seen = 0; //items.len() as u32;

        // operation
        let line: Vec<&str> = lines[i+2].split(" ").collect();
        let op = line[6].to_string();
        let op_a = line[5].to_string();
        let op_b = line[7].to_string();

        // divisor and throw recipient
        let divisor: u64 = lines[i+3].split(" ").collect::<Vec<&str>>()[5]
            .parse().unwrap();
        let dst_true: usize = lines[i+4].split(" ").collect::<Vec<&str>>()
            .last().unwrap()
            .parse().unwrap();
        let dst_false: usize = lines[i+5].split(" ").collect::<Vec<&str>>()
            .last().unwrap()
            .parse().unwrap();

        let m = Monkey {
            items,
            seen,
            op,
            op_a,
            op_b,
            divisor,
            dst_true,
            dst_false,
        };
        monkeys.push(m);
    }

    monkeys
}

fn play_shenanigans_part1(monkeys: &mut Vec<Monkey>, i: usize, divisor: u64) {
    // do the operation
    let dst_true = monkeys[i].dst_true;
    let dst_false = monkeys[i].dst_false;

    for _j in 0..monkeys[i].items.len() {
        monkeys[i].seen += 1;
        let old = monkeys[i].items.pop_front().unwrap();

        let a: u64 = match monkeys[i].op_a.as_str() {
            "old" => old,
            _     => monkeys[i].op_a.parse().unwrap(),
        };
        let b: u64 = match monkeys[i].op_b.as_str() {
            "old" => old,
            _     => monkeys[i].op_b.parse().unwrap(),
        };
        let new = match monkeys[i].op.as_str() {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            _   => 0
        } / divisor;

        // throw
        if new % monkeys[i].divisor == 0 {
            monkeys[dst_true].items.push_back(new);
        } else {
            monkeys[dst_false].items.push_back(new);
        }
    }
}

fn play_shenanigans_part2(monkeys: &mut Vec<Monkey>, i: usize, divisor: u64) {
    // do the operation
    let dst_true = monkeys[i].dst_true;
    let dst_false = monkeys[i].dst_false;
    //let gcd: u64 = monkeys.iter().map(|m| m.divisor).product();

    for _j in 0..monkeys[i].items.len() {
        monkeys[i].seen += 1;
        let old = monkeys[i].items.pop_front().unwrap();

        let a: u64 = match monkeys[i].op_a.as_str() {
            "old" => old,
            _     => monkeys[i].op_a.parse().unwrap(),
        };
        let b: u64 = match monkeys[i].op_b.as_str() {
            "old" => old,
            _     => monkeys[i].op_b.parse().unwrap(),
        };
        let new = match monkeys[i].op.as_str() {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            _   => 0
        } % divisor;

        // throw
        if new % monkeys[i].divisor == 0 {
            monkeys[dst_true].items.push_back(new);
        } else {
            monkeys[dst_false].items.push_back(new);
        }
    }
}

fn part1(monkeys: &mut Vec<Monkey>) -> u64 {
    for _i in 0..20 {
        for j in 0..monkeys.len() {
            play_shenanigans_part1(monkeys, j, 3);
        }
    }

    monkeys.sort_by_key(|m| m.seen);
    monkeys.reverse();

    // compute "monkey-business"
    monkeys[0].seen * monkeys[1].seen
}

fn part2(monkeys: &mut Vec<Monkey>) -> u64 {
    let gcd: u64 = monkeys.iter().map(|m| m.divisor).product();

    for _i in 0..10_000 {
        for j in 0..monkeys.len() {
            play_shenanigans_part2(monkeys, j, gcd);
        }
    }

    monkeys.sort_by_key(|m| m.seen);
    monkeys.reverse();

    // compute "monkey-business"
    monkeys[0].seen * monkeys[1].seen
}

#[allow(dead_code)]
pub fn solution() {
    let mut monkeys1 = build_monkeys();
    let mut monkeys2 = monkeys1.clone();

    let part1 = part1(&mut monkeys1);
    let part2 = part2(&mut monkeys2);

    println!("[11] Solution a: {}", part1);
    println!("[11] Solution b: {}", part2);
}
