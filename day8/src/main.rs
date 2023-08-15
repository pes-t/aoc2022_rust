use std::{fs, vec};

struct CheckResult {
    detected: bool,
    distance: usize,
}

fn check_left(grid: &Vec<Vec<u32>>, (row, col): (usize, usize)) -> CheckResult {
    let value = grid.get(row).unwrap().get(col).unwrap();
    let mut res: CheckResult = CheckResult { detected: (false), distance: (0) };
    let mut counter = 0;
    for x in (0..col).rev() {
        counter = x;
        let comp = grid.get(row).unwrap().get(x).unwrap();
        if comp >= value {
            res.detected = true;
            break;
        }
    }
    res.distance = col - counter;
    res
}
fn check_right(grid: &Vec<Vec<u32>>, (row, col): (usize, usize)) -> CheckResult {
    let value = grid.get(row).unwrap().get(col).unwrap();
    let mut res: CheckResult = CheckResult { detected: (false), distance: (0) };
    let mut counter = 0;
    for x in col+1..grid.len() {
        counter = x;
        let comp = grid.get(row).unwrap().get(x).unwrap();
        if comp >= value {
            res.detected = true;
            break;
        }
    }
    res.distance = counter - col;
    res
}
fn check_top(grid: &Vec<Vec<u32>>, (row, col): (usize, usize)) -> CheckResult {
    let value = grid.get(row).unwrap().get(col).unwrap();
    let mut res: CheckResult = CheckResult { detected: (false), distance: (0) };
    let mut counter = 0;
    for x in (0..row).rev() {
        counter = x;
        let comp = grid.get(x).unwrap().get(col).unwrap();
        if comp >= value {
            res.detected = true;
            break;
        }
    }
    res.distance = row - counter;
    res
}
fn check_down(grid: &Vec<Vec<u32>>, (row, col): (usize, usize)) -> CheckResult {
    let value = grid.get(row).unwrap().get(col).unwrap();
    let mut res: CheckResult = CheckResult { detected: (false), distance: (0) };
    let mut counter = 0;
    for x in row+1..grid.len() {
        counter = x;
        let comp = grid.get(x).unwrap().get(col).unwrap();
        if comp >= value {
            res.detected = true;
            break;
        }
    }
    res.distance = counter - row;
    res
}

fn main() {

    let input = fs::read_to_string("src/input.txt")
        .expect("File was not readable");
    let input: Vec<_> = input.split('\n').collect();

    let mut grid: Vec<Vec<u32>> = vec![]; //grid[i][j], i = row, j = col

    for (i, line) in (&input).iter().enumerate() {
        if line.is_empty() {break;}
        grid.push(vec![]);

        let heights: Vec<_> = line
            .chars()
            .map(|c| c.to_digit(10)
                .unwrap()
            ).collect();

        for height in heights {
            grid.get_mut(i).unwrap().push(height);
        }        
    }

    let dim = grid.len();
    let mut counter = 0;
    let mut max_scenic_score = 0;

    for row in 1..dim-1 { // dont look at the edges
        for col in 1..dim-1 {
            // println!("Tree at {},{} scenic score from the left? {}", row, col, check_left(&grid, (row, col)).distance);
            // println!("Tree at {},{} scenic score from the right? {}", row, col, check_right(&grid, (row, col)).distance);
            // println!("Tree at {},{} scenic score from the top? {}", row, col, check_top(&grid, (row, col)).distance);
            // println!("Tree at {},{} scenic score from the bottom? {}", row, col, check_down(&grid, (row, col)).distance);

            let visible = !check_left(&grid, (row, col)).detected 
                | !check_right(&grid, (row, col)).detected
                | !check_top(&grid, (row, col)).detected
                | !check_down(&grid, (row, col)).detected;
            counter += if visible {1} else {0};

            let scenic_score = check_left(&grid, (row, col)).distance 
                * check_right(&grid, (row, col)).distance
                * check_top(&grid, (row, col)).distance
                * check_down(&grid, (row, col)).distance;
            max_scenic_score = if max_scenic_score > scenic_score {max_scenic_score} else {scenic_score};
        }
    }
    
    println!("{:?}", counter + (dim * 2 + (dim - 2) * 2));
    println!("{:?}", max_scenic_score);

}
