use std::fs;

struct Number {
    pos: usize,
    value: i64,
}

fn init(key: i64) -> Vec<Number> {
    let numbers: Vec<Number> = fs::read_to_string("resources/20_input_00.txt")
        .unwrap()
        .split("\n")
        .filter(|l| *l != "")
        .enumerate()
        .map(|(i, n)| Number { pos: i, value: key * n.parse::<i64>().unwrap() })
        .collect();

    numbers
}

fn decrypt(numbers: &mut Vec::<Number>, rounds: usize) -> i64 {
    let len = numbers.len();

    for _round in 0..rounds {
        for i in 0..len {
            let index = numbers.iter().position(|n| n.pos == i).unwrap();
            let mut new_index = index as i64 + numbers[index].value;

            new_index = new_index.rem_euclid(len as i64 - 1);

            let n = numbers.remove(index);
            numbers.insert(new_index as usize, n);
        }
    }

    let zero = numbers.iter().position(|x| x.value == 0).unwrap();
    let a = numbers[(zero + 1000) % len].value;
    let b = numbers[(zero + 2000) % len].value;
    let c = numbers[(zero + 3000) % len].value;

    a + b + c
}

#[allow(dead_code)]
pub fn solution() {
    // part 1
    let key = 1;
    let rounds = 1;
    let mut numbers = init(key);
    let p1 = decrypt(&mut numbers, rounds);

    // part 2
    let key = 811_589_153;
    let rounds = 10;
    let mut numbers = init(key);
    let p2 = decrypt(&mut numbers, rounds);

    println!("[20] solution a: {}", p1);
    println!("[20] solution b: {}", p2);
}
