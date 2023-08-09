use std::{fs, vec};

fn main() {
    let input = fs::read_to_string("src/input.txt")
        .expect("File reading failed");
    let input: Vec<&str> = input.split('\n').collect();
    
    let set_index = input
        .iter()
        .position(|&x| x == String::from(""))
        .unwrap();
    let temp = &input[..set_index-1];
    let init_crate_state: Vec<_> = temp.iter().rev().collect();
    let move_insts = &input[set_index+1..];

    // Calc # of crates and initialize
    let num_crates: usize =(input[0].len() as f32 / 4 as f32).ceil() as usize; 
    let mut crates: Vec<Vec<char>> = vec![];

    for _ in 0..num_crates {
        crates.push(vec![]);
    }

    // Separate line into crate contents
    for line in &init_crate_state {
        let a: Vec<_> = line.chars().collect();

        for x in 0..num_crates {
            let contents = a.get(1 + 4*x);
            match contents {
                None => continue,
                Some(i) => {
                    match *i {
                        ' ' => continue,
                        _ => crates.get_mut(x).unwrap().push(*i)
                    }
                }
            };
        }
    }

    println!("{:?}", crates);


    // part 1 sol
    // for move_inst in move_insts {
    //     if move_inst.is_empty() {break};
    //     let inst: Vec<&str> = move_inst.split(' ').collect();
    //     let num_crates_moved: i32 = inst[1].parse().unwrap();
    //     let src_crate: usize = inst[3].parse::<usize>().unwrap() - 1;
    //     let dst_crate: usize = inst[5].parse::<usize>().unwrap() - 1;

    //     for _ in 0..num_crates_moved {
    //         let temp = crates.get_mut(src_crate).unwrap().pop().unwrap();
    //         crates.get_mut(dst_crate).unwrap().push(temp);
    //     }  
    // }

    // part 2 sol
    for move_inst in move_insts {
        if move_inst.is_empty() {break};
        let inst: Vec<&str> = move_inst.split(' ').collect();
        let num_crates_moved: usize = inst[1].parse().unwrap();
        let src_crate: usize = inst[3].parse::<usize>().unwrap() - 1;
        let dst_crate: usize = inst[5].parse::<usize>().unwrap() - 1;

        let mut swap_vector: Vec<char> = Vec::with_capacity(num_crates_moved);
        for _ in 0..num_crates_moved {
            let temp = crates.get_mut(src_crate).unwrap().pop().unwrap();
            swap_vector.push(temp);
        }  
        for _ in 0..num_crates_moved {
            let temp = swap_vector.pop().unwrap();
            crates.get_mut(dst_crate).unwrap().push(temp);
        }
    }


    println!("After moving\n");
    println!("{:?}", crates);

    for c in crates {
        println!("Top of Crate - {:?}", c.last().unwrap());
    }

}
