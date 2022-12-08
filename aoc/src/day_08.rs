use std::fs;

fn is_higher_top(matrix: &Vec<&[u8]>, row: usize, col: usize, value: u8) -> (bool, u64) {
    let mut higher = true;
    let mut view = 0;

    for i in (0..row).rev() {
        // part 2
        if value > matrix[i as usize][col] {
            view += 1;
        }

        if value <= matrix[i as usize][col] {
            higher = false;
            view += 1;
            break;
        }
    }

    (higher, view)
}

fn is_higher_left(matrix: &Vec<&[u8]>, row: usize, col: usize, value: u8) -> (bool, u64) {
    let mut higher = true;
    let mut view = 0;

    for j in (0..col).rev() {
        // part 2
        if value > matrix[row as usize][j] {
            view += 1;
        }

        if value <= matrix[row as usize][j] {
            higher = false;
            view += 1;
            break;
        }
    }

    (higher, view)
}

fn is_higher_bottom(matrix: &Vec<&[u8]>, row: usize, col: usize, value: u8) -> (bool, u64) {
    let mut higher = true;
    let mut view = 0;

    for i in row+1..matrix.len() {
        // part 2
        if value > matrix[i as usize][col] {
            view += 1;
        }

        if value <= matrix[i as usize][col] {
            higher = false;
            view += 1;
            break;
        }
    }

    (higher, view)
}

fn is_higher_right(matrix: &Vec<&[u8]>, row: usize, col: usize, value: u8) -> (bool, u64) {
    let mut higher = true;
    let mut view = 0;

    for j in col+1..matrix[0].len() {
        // part 2
        if value > matrix[row as usize][j] {
            view += 1;
        }

        if value <= matrix[row as usize][j] {
            higher = false;
            view += 1;
            break;
        }
    }

    (higher, view)
}

#[allow(dead_code)]
pub fn solution() {
    let input = fs::read_to_string("resources/08_input_00.txt")
        .expect("[-] Couldn't read the file");
    let grid: Vec<&[u8]> = input.split("\n")
        .filter(|l| *l != "")
        .map(|l| l.as_bytes())
        .collect();
    let height = grid.len();
    let width = grid[0].len();
    let mut visible = width * 2 + height * 2 - 4; // add the edges
    let mut max_view = 0;

    for i in 1..height-1 {
        let row = grid[i];

        for j in 1..width-1 {
            let tree = row[j];

            let (top, topv) = is_higher_top(&grid, i, j, tree);
            let (left, leftv) = is_higher_left(&grid, i, j, tree);
            let (right, rightv) = is_higher_right(&grid, i, j, tree);
            let (bottom, bottomv) = is_higher_bottom(&grid, i, j, tree);
            let view = topv * leftv * rightv * bottomv;

            if top || left || right || bottom {
                visible += 1;
            }

            if view > max_view {
                max_view = view;
            }
        }
    }

    println!("[08] solution a: {}", visible);
    println!("[08] solution b: {}", max_view);
}
