use std::fs;
use std::collections::HashMap;

fn update_parents_sizes(dirs: &mut HashMap<String, u64>) {
    for d in dirs.clone() {
        let parents: Vec<&str> = d.0.split("/").collect();
        let size = d.1;

        // update sizes of parent dirs
        for i in 1..parents.len() {
            let parent = &parents[0..parents.len()-i].join("/");
                *dirs.get_mut(parent).unwrap() += size.clone();
        }

        // updates the root
        if parents[0] != "" {
            *dirs.get_mut("").unwrap() += size;
        }
    }
}

fn compute_answers(dirs: &mut HashMap<String, u64>) -> (u64, u64) {
    let mut sum: u64 = 0;
    let mut smallest = u64::MAX;
    let max = 100_000;
    let available = 70_000_000;
    let target = 30_000_000;
    let unused = available - *dirs.get_mut("").unwrap();

    for d in dirs {
        let size = *d.1;

        // part 1
        if size < max {
            sum += size;
        }

        // part 2
        if size + unused > target && size < smallest {
            smallest = size;
        }
    }

    (sum, smallest)
}

#[allow(dead_code)]
pub fn solution() {
    let input = fs::read_to_string("resources/07_input_00.txt")
        .expect("[-] Couldn't read the file");
    let lines = input.split("\n");
    let mut wd = vec![]; // working dir
    let mut dirs: HashMap<String, u64> = HashMap::new();

    for l in lines.filter(|l| *l != "") {
        let split: Vec<&str> = l.split(" ").collect();

        if l.starts_with("$") {
            // creates a vec of folders forming a path
            match split[1] {
                "ls" => (),
                "cd" => match split[2] {
                    ".." => { wd.pop(); },
                    "/"  => { wd.clear(); },
                    _    => { wd.push(split[2].to_string()); }
                },
                _    => panic!("WTF!?")
            }
        } else {
            let current = wd.join("/");
            let mut fsize: u64 = 0;

            // add the path to the hashmap
            if !l.starts_with("dir") {
                let tmp = l.split(" ").collect::<Vec<&str>>()[0];
                fsize = tmp.parse().unwrap();
            }

            // increment the folder's total size
            if dirs.contains_key(&current) {
                *dirs.get_mut(&current).unwrap() += fsize;
            } else {
                dirs.insert(current.clone(), fsize);
            }
        }
    }

    update_parents_sizes(&mut dirs);

    let (sum, smallest) = compute_answers(&mut dirs);

    println!("[07] solution a: {}", sum);
    println!("[07] solution b: {}", smallest);
}

