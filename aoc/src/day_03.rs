use std::fs;

#[allow(dead_code)]
pub fn solution() {
    let input = fs::read_to_string("resources/03_input_00.txt")
        .expect("[-] Couldn't read the file");
    let rucksacks = input.split("\n");
    let mut sum_p1: u64 = 0;
    let mut sum_p2: u64 = 0;

    // part 1
    for r in rucksacks.clone().filter(|e| *e != "") {
        let items = r.as_bytes();

        for i in &items[0..items.len()/2] {
            if items[items.len()/2..].contains(&i) {
                if *i <= 0x5a {
                    sum_p1 += (*i - 0x40 + 26) as u64;
                } else {
                    sum_p1 += (*i - 0x60) as u64;
                }
                break;
            }
        }
    }

    // part 2
    let sacks: Vec<&[u8]> = rucksacks.filter(|s| *s != "")
        .map(|items| items.as_bytes())
        .collect();

    for i in (0..sacks.len() - 2).step_by(3) {
        let items = sacks[i];

        for j in 0..items.len() {
            if sacks[i+1].contains(&items[j]) && sacks[i+2].contains(&items[j]) {
                if items[j] <= 0x5a {
                    sum_p2 += (items[j] - 0x40 + 26) as u64;
                } else {
                    sum_p2 += (items[j] - 0x60) as u64;
                }
                break;
            }
        }
    }

    println!("[03] Solution a: {}", sum_p1);
    println!("[03] Solution b: {}", sum_p2);
}
