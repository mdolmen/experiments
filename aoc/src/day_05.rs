use std::fs;

#[allow(dead_code)]
pub fn solution() {
    let input = fs::read_to_string("resources/05_input_00.txt")
        .expect("[-] Couldn't read the file");
    let lines = input.split("\n");
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut stacks_p2: Vec<Vec<char>> = Vec::new();
    let mut init = false;
    let mut msg_p1 = String::from("");
    let mut msg_p2 = String::from("");

    for l in lines.filter(|l| *l != "") {
        let line = l.as_bytes();

        // init the stacks
        if !init {
            // the first one will not be used to have the same index when applying instruction
            // (i.e. start indexing at 1)
            let nb_stacks = (line.len()+1) / 4 + 1;
            for _i in 0..nb_stacks {
                stacks.push(vec![]);
            }
            init = true;
        }

        // fill the stacks
        if l.contains('[') {
            for i in (1..l.len()).step_by(4) {
                if line[i] != ' ' as u8 {
                    let index = i / 4 + 1;
                    //println!("stack {index}: push {:?}", line[i]);
                    stacks[index].push(line[i] as char);
                }
            }
        }

        // reverse the stacks upon reaching the line with indices
        if line[1] as char == '1' {
            for s in &mut stacks {
                s.reverse();
            }

            stacks_p2 = stacks.clone();
        }

        // apply instructions
        if line[0] as char == 'm' {
            let tmp: Vec<&str> = l.split(" ").collect();
            let number: usize = tmp[1].parse().unwrap();
            let src: usize = tmp[3].parse().unwrap();
            let dst: usize = tmp[5].parse().unwrap();

            // part 1
            for _i in 0..number {
                let c = stacks[src].pop().unwrap();
                stacks[dst].push(c);
            }

            // part 2
            if number > 1 {
                let len = stacks_p2[src].len();
                let mut crates = stacks_p2[src][len-number..].to_vec();
                stacks_p2[dst].append(&mut crates);
                stacks_p2[src].truncate(len-number);
            } else {
                match stacks_p2[src].pop() {
                    Some(c) => stacks_p2[dst].push(c),
                    None    => ()
                }
            }
        }
    }

    for i in 1..stacks.len() {
        msg_p1.push(stacks[i][stacks[i].len() - 1]);
        msg_p2.push(stacks_p2[i][stacks_p2[i].len() - 1]);
    }

    println!("[05] Solution a: {}", msg_p1);
    println!("[05] Solution b: {}", msg_p2);
}
