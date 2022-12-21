use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Monkey {
    ready: bool,
    value: i64,
    a: String,
    b: String,
    op: char
}

impl Monkey {
    pub fn get_value(&self) -> Option<i64> {
        if self.ready {
            Some(self.value)
        } else {
            None
        }
    }
}

fn init(is_p2: bool) -> HashMap<String, Monkey> {
    let input = fs::read_to_string("resources/21_input_00.txt")
        .expect("[-] Couldn't read the file");
    let lines: Vec<Vec<&str>> = input.split("\n")
        .filter(|l| *l != "")
        .map(|m| m.split(":")
            .map(|e| e.trim())
            .collect())
        .collect();
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();

    // build monkeys
    for l in lines {
        let name = l[0].to_string();
        let mut ready = false;
        let mut value = 0;
        let mut a = String::from("");
        let mut b = String::from("");
        let mut op = '.';

        match l[1].parse::<i64>() {
            Ok(v) => {
                ready = true;
                value = v;
            },
            _     => {
                let tmp: Vec<&str> = l[1].split(" ").collect();
                a = tmp[0].to_string();
                b = tmp[2].to_string();
                op = tmp[1].chars().nth(0).unwrap();
            }
        };

        // part 2
        if is_p2 && name == String::from("root") {
            op = '=';
        }

        monkeys.insert(
            name,
            Monkey {
                ready: ready,
                value: value,
                a: a,
                b: b,
                op: op,
            }
        );
    }

    monkeys
}

/// Returns the tuple ('a', 'result'). 'b' can be deduced from that (for part  2).
fn wait_monkey(monkeys: &HashMap<String, Monkey>, name: &String) -> (i64, i64) {
    let m = monkeys.get(name).unwrap();
    let mut a = 0;
    let monkey_yells;
    let b;

    (a, monkey_yells) = match m.get_value() {
        // the monkey its result, we're done
        Some(value) => (a, value),
        // wait for it...
        _       => {
            (_, a) = wait_monkey(&monkeys, &m.a);
            (_, b) = wait_monkey(&monkeys, &m.b);
            let result = match m.op {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                '/' => a / b,
                '=' => a + b, // part 2
                _   => panic!("wait_monkey: WTF!?"),
            };

            (a, result)
        }
    };

    return (a, monkey_yells);
}

/// Brute force value for 'humn'...
fn solve(monkeys: &mut HashMap<String, Monkey>, a: i64, result: i64) -> i64 {
    let root = String::from("root");
    let humn = String::from("humn");
    let b = result - a;
    let mut target = b;
    let mut guess = 0;

    // identify which of 'a' or 'b' depends on 'humn'
    monkeys.get_mut(&humn).unwrap().value = 123;
    let (new_a, _) = wait_monkey(&monkeys, &root);
    if new_a == a {
        target = a;
    }

    // binary search
    let mut min: i64 = 0;
    let mut max: i64 = 10_000_000_000_000;
    while min < max { 
        // update 'humn' monkey
        let mid = (max + min) / 2;
        monkeys.get_mut(&humn).unwrap().value = mid;

        let (new_a, new_res) = wait_monkey(&monkeys, &root);
        let mut cmp = new_res - (new_res - new_a); // new_res - b: looking for b
        if target == a {
            cmp = new_res - new_a; // looking for a
        }

        //println!("{min} {max} {target}, {cmp}, {mid}, {}", cmp-target);

        if cmp == target {
            guess = mid;
            break;
        }

        // update search range
        if cmp - target < 0 {
            max = mid;
        } else {
            min = mid;
        }
    }

    guess
}

#[allow(dead_code)]
pub fn solution() {
    let root = String::from("root");
    let monkeys = init(false);
    let (a, p1) = wait_monkey(&monkeys, &root);

    // we don't care of having the value 'humn' has to yell in the order of the game, just compute
    // what it should be based on results from part 1
    let mut monkeys = init(true);
    let p2 = solve(&mut monkeys, a, p1);

    println!("[21] solution a: {}", p1);
    println!("[21] solution b: {}", p2);
}
