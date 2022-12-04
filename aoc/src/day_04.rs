use std::fs;

#[allow(dead_code)]
pub fn solution() {
    let input = fs::read_to_string("resources/04_input_00.txt")
        .expect("[-] Couldn't read the file");
    let lines = input.split("\n");
    let pairs: Vec<Vec<Vec<u64>>> = lines.filter(|l| *l != "")
        .map(|p| p.split(",")
            .map(|e| e.split("-")
                .map(|x| x.parse::<u64>().unwrap())
                .collect())
            .collect())
        .collect();
    let mut nb_pairs_p1 = 0;
    let mut nb_pairs_p2 = 0;

    // part 1: complete overlap
    for p in pairs.clone() {
        let a = &p[0];
        let b = &p[1];

        if (a[0] >= b[0] && a[1] <= b[1]) || (b[0] >= a[0] && b[1] <= a[1]) {
            nb_pairs_p1 += 1;
        }
    }

    // part 2: any overlap
    for p in pairs {
        let a = &p[0];
        let b = &p[1];

        if (a[0] >= b[0] && a[0] <= b[1]) || (a[1] >= b[0] && a[1] <= b[1]) 
            || (b[0] >= a[0] && b[0] <= a[1]) || (b[1] >= a[0] && b[1] <= a[1]) 
        {
            nb_pairs_p2 += 1;
        }
    }

    println!("[04] Solution a: {}", nb_pairs_p1);
    println!("[04] Solution b: {}", nb_pairs_p2);
}
