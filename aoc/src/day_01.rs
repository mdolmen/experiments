use std::{cmp, fs};

pub fn solution() {
    //let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";

    let input = fs::read_to_string("resources/01_input_00.txt")
        .expect("[-] Couldn't read the file");
    let mut max = vec![0; 3];
    let mut sum = 0;
    let items = input.split("\n");

    for item in items {
        if item == "" {
            let min = max.iter().min().unwrap();
            let index = max.iter().position(|e| e == min).unwrap();
            max[index] = cmp::max(*min, sum);
            sum = 0;
            continue;
        }

        sum += item.parse::<u32>().unwrap();
    }

    println!("[01] Solution a: {}", max.iter().max().unwrap());
    println!("[01] Solution b: {}", max.iter().sum::<u32>());
}
