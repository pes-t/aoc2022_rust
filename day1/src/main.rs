// use core::num;
// use std::env;
use std::fs;

fn find_min(v: &Vec<i32>) -> (usize, i32) {
    let mut ind = 0;
    let mut min_val = i32::MAX;
    for (i, x) in v.iter().enumerate() {
        if *x < min_val {
            min_val = *x;
            ind = i;
        }
    }
    (ind, min_val)
}

fn main() {

    // Read in file and separate via newline
    let input = fs::read_to_string("src/input.txt")
        .expect("File reading failed");
    let input: Vec<&str> = input.split('\n').collect();


    let mut calorie_count = 0;
    let mut highest_calorie_count = -1;
    let mut three_highest_calories = vec![-1, -1, -1];
    for item in input {
        if item == "" {
            highest_calorie_count = if calorie_count > highest_calorie_count {calorie_count} else {highest_calorie_count};
            
            let min_ind;
            let min;
            (min_ind, min) = find_min(&three_highest_calories);
            if calorie_count > min {
                three_highest_calories[min_ind] = calorie_count;
            }
            
            calorie_count = 0;
            continue;
        }
        let calories: i32 = item.parse()
            .expect("Not able to convert to int");
        calorie_count += calories;
    }
    println!("{:?}", highest_calorie_count);
    // println!("{:?}", three_highest_calories);
    let temp = three_highest_calories.iter().fold(0, |sum, x| sum + (*x as i32));
    let temp2: i32 = three_highest_calories.iter().sum();
    println!("{:?}", temp);
    println!("{:?}", temp2);

}
