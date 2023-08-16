use std::{fs, vec, collections::{VecDeque, btree_set::Difference}};
use std::time::Instant;

struct CheckResult {
    detected: bool,
    distance: usize,
}

#[derive(PartialEq)]
enum Direction {
    Backwards,
    Forwards,
    Leftwards,
    Rightwards,
}

fn get_vector(grid: &Vec<Vec<u32>>, (row, col): (usize, usize), direction: Direction) -> Vec<&u32> {
    let mut flat_map: VecDeque<_> = grid.iter().flat_map(|row| row.iter()).collect();
    let grid_dim = grid.len();
    let offset = row * grid_dim + col;

    let res: Vec<_> = match direction {
        Direction::Backwards => {flat_map.drain(0..=offset).collect()},
        Direction::Forwards => {flat_map.drain(offset..).collect()},
        Direction::Rightwards => {flat_map.drain(offset..grid_dim*(row + 1) as usize).collect()},
        Direction::Leftwards => {flat_map.drain(grid_dim*row..=offset).rev().collect()},
    };
    let res = if direction == Direction::Backwards {res.into_iter().rev().step_by(grid_dim).collect()} 
        else if direction == Direction::Forwards{res.into_iter().step_by(grid_dim).collect()}
        else {res};
    res
}

fn check_visibility(trees: Vec<&u32>) -> CheckResult {
    let mut res = CheckResult {detected: false, distance: 0};
    let comp = trees[0];
    let mut distance = 0;
    for (i, elem) in trees.iter().enumerate() {
        if i == 0 {continue;}
        distance += 1;
        if *elem >= comp {
            res.detected = true;
            break;
        }
    }
    res.distance = distance;
    res
}



fn main() {
    let start_time = Instant::now();

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

    // let directions = [Direction::Backwards, Direction::Forwards, Direction::Leftwards, Direction::Rightwards];
    let mut visibility_counter = 0;
    let mut max_scenic_score = 0;
    let grid_dim = grid.len();
    for (row, grid_row) in grid.iter().enumerate() {
        for (col, _) in grid_row.iter().enumerate() {
            if (row == 0) | (col == 0) | (row == grid_dim-1) | (col == grid_dim-1){continue};
            let left_visibility = check_visibility(get_vector(&grid, (row, col), Direction::Leftwards));
            let right_visibility = check_visibility(get_vector(&grid, (row, col), Direction::Rightwards));
            let forwards_visibility = check_visibility(get_vector(&grid, (row, col), Direction::Forwards));
            let downwards_visibility = check_visibility(get_vector(&grid, (row, col), Direction::Backwards));
            let visible = !left_visibility.detected | !right_visibility.detected | !forwards_visibility.detected | !downwards_visibility.detected;
            let scenic_score = left_visibility.distance * right_visibility.distance * forwards_visibility.distance * downwards_visibility.distance; 
            visibility_counter += if visible {1} else {0};
            max_scenic_score = if scenic_score > max_scenic_score {scenic_score} else {max_scenic_score};
        
        
            // println!("element at ({}{}) is visible? {:?}",row, col, visible);
            // println!("max scenic sore at ({}{}){:?}", row, col, max_scenic_score);
            // println!("scenic score at ({}{}) is {}*{}*{}*{}", row, col, left_visibility.distance, right_visibility.distance, forwards_visibility.distance, downwards_visibility.distance)
        
        }
    }
    println!("{:?}", visibility_counter + grid_dim*2 + (grid_dim-2)*2);
    println!("{:?}", max_scenic_score);

    let end_time = Instant::now();
    println!("Time Elapsed {:?}", end_time - start_time);
}
