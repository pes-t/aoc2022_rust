use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

fn priority_map(a: char) -> i32 {
    let res: i32;
    match a {
        'a' => res = 1,
        'b' => res = 2,
        'c' => res = 3,
        'd' => res = 4,
        'e' => res = 5,
        'f' => res = 6,
        'g' => res = 7,
        'h' => res = 8,
        'i' => res = 9,
        'j' => res = 10,
        'k' => res = 11,
        'l' => res = 12,
        'm' => res = 13,
        'n' => res = 14,
        'o' => res = 15,
        'p' => res = 16,
        'q' => res = 17,
        'r' => res = 18,
        's' => res = 19,
        't' => res = 20,
        'u' => res = 21,
        'v' => res = 22,
        'w' => res = 23,
        'x' => res = 24,
        'y' => res = 25,
        'z' => res = 26,        
        'A' => res = 27,
        'B' => res = 28,
        'C' => res = 29,
        'D' => res = 30,
        'E' => res = 31,
        'F' => res = 32,
        'G' => res = 33,
        'H' => res = 34,
        'I' => res = 35,
        'J' => res = 36,
        'K' => res = 37,
        'L' => res = 38,
        'M' => res = 39,
        'N' => res = 40,
        'O' => res = 41,
        'P' => res = 42,
        'Q' => res = 43,
        'R' => res = 44,
        'S' => res = 45,
        'T' => res = 46,
        'U' => res = 47,
        'V' => res = 48,
        'W' => res = 49,
        'X' => res = 50,
        'Y' => res = 51,
        'Z' => res = 52,
        _ => res = -1
    }
    res
}

fn main() {
    let input = fs::read_to_string("src/input.txt")
        .expect("File reading failed");
    let input: Vec<&str> = input.split('\n').collect();
    
    // Parse input into tuple of (vec<char>, vec<char>) for comparison later
    let mut rucksack_contents: Vec<(Vec<char>, Vec<char>)> = vec![];
    for line in &input {
        if line.is_empty() { continue };

        let a: Vec<char> = line.chars().collect();
        let length = a.len();
        rucksack_contents.push( (a[0 .. length/2].to_vec(), a[length/2 .. ].to_vec()) );
    }

    // Put every seen element into hashmap, and if key already exists on the second go around, the element has been repeated in both 
    let mut priority_sum = 0;
    for rucksack in &rucksack_contents {
        let compartment1 = &rucksack.0;
        let compartment2 = &rucksack.1;

        let mut seen_items: HashMap<char, bool> = HashMap::new();
        for item in compartment1 {
            seen_items.insert(*item, true);
        }
        for item in compartment2 {
            if seen_items.contains_key(item) {
                // println!("{} {} is in both compartments", *item, priority_map(*item));
                priority_sum += priority_map(*item);
                break;
            }
        }


    }

    println!("priority sum - {:?}", priority_sum);


    // Take three strings at a time, look through them individually.
    // Have 2 maps, one to see if the char is seen for the first time (per iteration), and one to count total number of strings the char appears in
    let input_size = input.len()-1;
    let mut i = 0;
    let step = 3;
    let mut p2_priority_sum = 0;
    while i < input_size {
        let mut j = 0;
        let mut item_count: HashMap<char, i32> = HashMap::new();
        while j < step {
            // Look at rucksacks 3 at a time
            let mut seen_items: HashMap<char, bool> = HashMap::new();

            for item in input[i+j].chars() {
                if (!seen_items.contains_key(&item)) {
                    // println!("{}", item);
                    seen_items.insert(item, true);
                    let times_seen = match item_count.get(&item) {
                        Some(x) => {x},
                        None => {&0},
                    };
                    item_count.insert(item, times_seen + 1);
                }
            }
        j += 1;
        }
        // println!("{:?}", item_count);
        let mut max_seen = -1;
        let mut max_seen_key = '_';
        for (key, value) in &item_count {
            if *value > max_seen {
                max_seen = *value;
                max_seen_key = *key;
            }
        }
        // println!("key w/ largest value was {:?}", max_seen_key);
        p2_priority_sum += priority_map(max_seen_key);

        i += 3;
    }

    println!("part 2 priority sum - {}", p2_priority_sum);





    println!("Hello, world!");
}
