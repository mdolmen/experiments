use std::fs;

#[allow(dead_code)]
pub fn solution() {
    //let input = "A Y\nB X\nC Z\nA Z";
    let input = fs::read_to_string("resources/02_input_00.txt")
        .expect("[-] Couldn't read the file");
    let rounds = input.split("\n");
    let shapes = vec![1, 2, 3];
    let results = vec![0, 3, 6];
    let mut score_p1 = 0;
    let mut score_p2 = 0;

    // part 1
    for r in rounds.clone() {
        if r == "" {
            break;
        }

        let a: i32 = r.as_bytes()[0] as i32;
        let b: i32 = r.as_bytes()[2] as i32 - 0x17;
        let play = a - b;

        score_p1 += shapes[(b % 0x41) as usize];
        score_p1 += match play {
            0  => results[1], // draw
            1  => results[0], // lose
            2  => results[2], // win
            -1 => results[2], // win
            -2 => results[0], // lose
            _  => 0
        };
        //println!("{a} {b} {tmp}+{}", shapes[(b%0x41) as usize]);
    }

    // part 2
    for r in rounds {
        if r == "" {
            break;
        }

        let a = r.as_bytes()[0];
        let b = r.as_bytes()[2];
        let result = b - 0x58;
        let shape = match b - 0x58 {
            0 => (a - 0x41 + 2) % results.len() as u8,  // X
            1 => a - 0x41,      // Y
            2 => (a - 0x41 + 1) % results.len() as u8,  // Z
            _ => 0
        };
        //println!("{a} {b} {result} {shape}");
        score_p2 += results[result as usize] + shapes[shape as usize];
    }

    println!("[02] Solution a: {}", score_p1);
    println!("[02] Solution b: {}", score_p2);
}
