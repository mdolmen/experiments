///
/// Original solution: https://www.youtube.com/watch?v=w7m48_uCvWI
///

use std::{fs, cmp};

#[derive(Debug, Clone, Default, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn parse_sensor(str_x: &str, str_y: &str) -> Self {
        let len_x = str_x.len();
        let len_y = str_y.len();

        Self {
            x: str_x[2..len_x-1].parse().unwrap(),
            y: str_y[2..len_y-1].parse().unwrap(),
        }
    }

    fn parse_beacon(str_x: &str, str_y: &str) -> Self {
        let len_x = str_x.len();

        Self {
            x: str_x[2..len_x-1].parse().unwrap(),
            y: str_y[2..].parse().unwrap(),
        }
    }

    // Manhattan distance
    fn dist(a: &Point, b: &Point) -> i64 {
        (a.x - b.x).abs() + (a.y - b.y).abs()
    }
}

fn init() -> (Vec<Point>, Vec<Point>) {
    //let input = fs::read_to_string("resources/test.txt")
    //    .expect("[-] Couldn't read the file");
    let input = fs::read_to_string("resources/15_input_00.txt")
        .expect("[-] Couldn't read the file");
    let lines = input
        .split("\n")
        .filter(|l| *l != "");
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();

    for line in lines {
        let elems: Vec<&str> = line.split(" ")
            .filter(|l| *l != "")
            .collect();

        // sensor coordinates
        sensors.push(Point::parse_sensor(&elems[2], &elems[3]));
        beacons.push(Point::parse_beacon(&elems[8], &elems[9]));
    }

    (sensors, beacons)
}

fn solve_p1(sensors: &Vec<Point>, beacons: &Vec<Point>) -> i64 {
    let mut dists = Vec::new();
    let mut intervals = Vec::new();
    let mut max_x = 0;
    let len = sensors.len();
    let y = 10;

    for i in 0..len {
        dists.push( Point::dist(&sensors[i], &beacons[i]) );
    }

    for i in 0..len {
        // Compute the interval between the axis of the sensor and of the
        // beacons. I.e. get the numbers of X overlapped by the square around
        // the sensor (cf. Manhattan geometry).
        let dx = dists[i] - (sensors[i].y - y).abs() as i64;

        // If there is no overlapping, |sy - Y| while be greater than the manh.
        // distance, so dx will be negative.
        if dx <= 0 {
            continue;
        }

        intervals.push((cmp::max(0, sensors[i].x-dx), sensors[i].x+dx));

        max_x = cmp::max(max_x, sensors[i].x+dx);
    }

    // Get unique invalid X.
    let mut forbidden = vec![0; 40_000_000];
    for i in 0..intervals.len() {
        for x in intervals[i].0..intervals[i].1 {
            forbidden[x as usize] = 1;
        }
    }

    forbidden.into_iter().filter(|f| *f == 1).collect::<Vec<i8>>().len() as i64
}

///
/// Does not work for the test inputs but works with the real ones...
///
fn solve_p2(sensors: &Vec<Point>, beacons: &Vec<Point>) -> i64 {
    let mut dists = Vec::new();
    let mut neg_lines = Vec::new();
    let mut pos_lines = Vec::new();
    let mut neg = None;
    let mut pos = None;
    let mut len = sensors.len();

    // Get all manhattan distances.
    for i in 0..len {
        dists.push( Point::dist(&sensors[i], &beacons[i]) );
    }

    // Get all the slopes.
    for i in 0..len {
        let d = dists[i];
        neg_lines.push(sensors[i].x + sensors[i].y - d);
        neg_lines.push(sensors[i].x + sensors[i].y + d);
        pos_lines.push(sensors[i].x - sensors[i].y - d);
        pos_lines.push(sensors[i].x - sensors[i].y + d);
    }

    len *= 2;
    'outer: for i in 0..len {
        for j in i+1..len {
            // Look for two slopes with an interval of 2, meaning the beacon
            // might be between them.
            let a = neg_lines[i];
            let b = neg_lines[j];
            if (a - b).abs() == 2 {
                neg = Some(cmp::min(a, b) + 1);
            }

            let a = pos_lines[i];
            let b = pos_lines[j];
            if (a - b).abs() == 2 {
                pos = Some(cmp::min(a, b) + 1);
            }

            // We assume there is at least one solution. We assume there is only
            // one...
            if !neg.is_none() && !pos.is_none() {
                break 'outer;
            }
        }
    }

    // Solution to the system of equation:
    //  x - y = pos
    //  x + y = neg
    let x = (pos.unwrap() + neg.unwrap()) / 2;
    let y = (neg.unwrap() - pos.unwrap()) / 2;
    println!("x = {}, y = {}", x, y);

    // Tuning frequency
    x * 4_000_000 + y
}

pub fn solution() {
    let (sensors, beacons) = init();
    let p1 = solve_p1(&sensors, &beacons);
    let p2 = solve_p2(&sensors, &beacons);

    println!("[15] Solution a: {}", p1);
    println!("[15] Solution b: {}", p2);
}
