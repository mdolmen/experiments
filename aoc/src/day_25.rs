use std::fs;

fn snafu_to_int(snafu: &Vec<char>) -> i64 {
    let mut number: i64 = 0;

    for (i, digit) in snafu.into_iter().rev().enumerate() {
        let d = match digit {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _   => panic!("snafu_to_int: WTF!?"),
        };

        number += d * 5_i64.pow(i as u32);
    }

    number
}

fn int_to_snafu(number: i64) -> String {
    let mut snafu: String = String::new();
    let mut tmp = number;

    while tmp > 0 {
        let d = ((tmp + 2) % 5) - 2;
        let c = match d {
            -2 => '=',
            -1 => '-',
            0  => '0',
            1  => '1',
            2  => '2',
            _   => panic!("snafu_to_int: WTF!?"),
        };
        tmp -= d;
        tmp /= 5;

        snafu.push(c);
    }

    snafu.chars().rev().collect()
}

fn solve(list: &Vec<Vec<char>>) -> String {
    let mut sum: i64 = 0;

    for snafu in list {
        sum += snafu_to_int(&snafu);
    }

    int_to_snafu(sum)
}

fn init() -> Vec<Vec<char>> {
    //let input = fs::read_to_string("resources/test.txt")
    let input = fs::read_to_string("resources/25_input_00.txt")
        .expect("[-] Couldn't read the file");
    let numbers: Vec<Vec<char>> = input.split("\n")
        .filter(|l| *l != "")
        .map(|l| l.chars().collect())
        .collect();

    numbers
}

#[allow(dead_code)]
pub fn solution() {
    let numbers = init();
    let p1 = solve(&numbers);

    println!("[25] solution a: {}", p1);
    //println!("[22] solution b: {}", p2);
}
