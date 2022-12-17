use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};

///
/// Optimization from: https://www.youtube.com/watch?v=bLMj50cpOug
///

fn init() -> (HashMap<String, i32>, HashMap<String, Vec<String>>) {
    //let input = fs::read_to_string("resources/test.txt")
    //    .expect("[-] Couldn't read the file");
    let input = fs::read_to_string("resources/16_input_00.txt")
        .expect("[-] Couldn't read the file");
    let lines = input
        .split("\n")
        .filter(|l| *l != "");
    let mut valves_flow = HashMap::new();
    let mut tunnels = HashMap::new();

    for line in lines {
        // Valve and their flow
        let parts: Vec<&str> = line.split(' ').collect();
        let valve = parts[1].to_string();
        let tmp = parts[4]
            .split('=')
            .collect::<Vec<&str>>()[1];
        let flow: i32 = tmp[0..tmp.len()-1].parse().unwrap();

        // Tunnels
        let dests_str = parts[9..]
            .to_vec()
            .join("");
        let dests: Vec<String> = dests_str
            .split(',')
            .map(|s| s.to_string())
            .collect();

        valves_flow.insert(valve.clone(), flow);
        tunnels.insert(valve, dests);
    }

    (valves_flow, tunnels)
}

fn solve_p1(
    valves_flow: HashMap<String, i32>,
    tunnels: HashMap<String, Vec<String>>
) -> (i32, i32) {
    let mut dists: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let mut nonempty: Vec<String> = Vec::new();

    // Map each node to the list of non-empty nodes it can reach.
    for (valve, &valve_flow) in &valves_flow {
        if valve != "AA" && valve_flow == 0 {
            continue;
        }

        if valve != "AA" {
            nonempty.push(valve.clone());
        }

        let mut visited: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<(&String, i32)> = VecDeque::new();
        queue.push_back((valve, 0));

        let mut valve_distances: HashMap<String, i32> = HashMap::new();
        valve_distances.insert(valve.clone(), 0);
        //valve_distances.insert("AA".to_string(), 0);
        dists.insert(valve.clone(), valve_distances);

        while let Some((position, distance)) = queue.pop_front() {
            for neighbor in tunnels.get(position).unwrap() {
                if visited.contains(neighbor) {
                    continue;
                }
                visited.insert(neighbor.clone());
                if valves_flow.contains_key(neighbor) && valves_flow[neighbor] != 0 {
                    dists.get_mut(valve).unwrap().insert(neighbor.clone(), distance + 1);
                }
                queue.push_back((neighbor, distance + 1));
            }
        }

        dists.get_mut(valve).unwrap().remove(valve);
        //if valve != "AA" {
        //    dists.get_mut(valve).unwrap().remove("AA");
        //}
    }

    let (p1, p2) = find_max_pressure(30, "AA", 0, &dists, &valves_flow, &nonempty);

    (p1, p2)
}

fn dfs<'a>(
    time: i32,
    valve: &'a str,
    seen: u32,
    indices: &HashMap<String, usize>,
    dists: &'a HashMap<String, HashMap<String, i32>>,
    valves: &HashMap<String, i32>,
    cache: &mut HashMap<(i32, &'a str, u32), i32>,
) -> i32 {
    if let Some(&result) = cache.get(&(time, valve, seen)) {
        return result;
    }

    let mut max_pressure = 0;

    if let Some(neighbors) = dists.get(valve) {
        for (neighbor, &neighbor_distance) in neighbors.iter() {
            // keep track of seen neighbors
            let bit = 1 << indices[neighbor];
            if seen & bit != 0 {
                continue;
            }

            // keep max pressure that will be released in the remaining time
            let remtime = time - neighbor_distance - 1;
            if remtime <= 0 {
                continue;
            }
            max_pressure = max_pressure.max(
                dfs(
                    remtime,
                    neighbor,
                    seen | bit,
                    indices,
                    dists,
                    valves,
                    cache,
                ) + valves[neighbor] * remtime,
            );
        }
    }

    cache.insert((time, valve, seen), max_pressure);

    max_pressure
}

fn find_max_pressure(
    time: i32,
    start: &str,
    seen: u32,
    dists: &HashMap<String, HashMap<String, i32>>,
    valves: &HashMap<String, i32>,
    nonempty: &Vec<String>,
) -> (i32, i32) {
    let mut indices: HashMap<String, usize> = HashMap::new();

    for (index, element) in nonempty.iter().enumerate() {
        indices.insert(element.clone(), index);
    }

    // Map of (time, valve, seen of seen neighbors) to max_pressure
    let mut cache: HashMap<(i32, &str, u32), i32> = HashMap::new();

    //
    // PART 1
    //

    let result_p1 = dfs(
        time,
        start,
        seen,
        &indices,
        &dists,
        &valves,
        &mut cache,
    );

    //
    // PART 2: take some time (< 1min)
    //

    let mut result_p2 = 0;

    // We split the valves in 2 partitions
    let partition: u32 = (1 << nonempty.len()) - 1;

    // Test all possible partition combinations
    for i in 0..partition/2 {
        result_p2 = result_p2.max(
            dfs(
                26,
                start,
                i,
                &indices,
                &dists,
                &valves,
                &mut cache,
            ) + dfs(
                26,
                start,
                partition ^ i,
                &indices,
                &dists,
                &valves,
                &mut cache,
            ),
        );
    }

    (result_p1, result_p2)
}

pub fn solution() {
    let (valves, tunnels) = init();

    let (p1, p2) = solve_p1(valves, tunnels);

    println!("[16] Solution a: {}", p1);
    println!("[16] Solution b: {}", p2);
}
