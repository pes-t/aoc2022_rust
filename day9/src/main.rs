use std::{fs, vec, collections::HashSet, hash::Hash, fmt};
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Inst {
    direction: Orientations,
    count: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Orientations {
    Up, Down, Left, Right,
}

fn next_move_t2(front: &Position, rear: &Position) -> (i32, i32) {
    let x_diff: i32 = front.x - rear.x;
    let y_diff: i32 = front.y - rear.y;

    let x_move = if x_diff > 0 {1} else if x_diff < 0 {-1} else {0};
    let y_move = if y_diff > 0 {1} else if y_diff < 0 {-1} else {0};
    // println!("\n{:?} {:?} - move in ({},{})", front, rear, x_move, y_move);
    (x_move, y_move)
}

fn distance_t2(front: &Position, back: &Position) -> f32 {
    let distance: f32 = ((front.x - back.x).pow(2) + (front.y - back.y).pow(2)) as f32;
    (distance).sqrt()
}

fn print_board(rope: &Vec<Position>) {
    let min_x = rope.iter().map(|x| x.x).min().unwrap();
    let max_x = rope.iter().map(|x| x.x).max().unwrap();
    let min_y = rope.iter().map(|x| x.y).min().unwrap();
    let max_y = rope.iter().map(|x| x.y).max().unwrap();

    // let max_x = 14;
    // let max_y = 15;
    // let min_x = -11;
    // let min_y = -5;

    println!("{:?} {:?} {:?} {:?}", min_x, max_x, min_y, max_y);

    let locations: HashSet<(i32, i32)> = rope.iter().map(|x| (x.x, x.y)).collect();
    for y in (min_y..max_y).rev() {
        for x in min_x..max_x {
            if locations.contains(&(x,y)) {print!("x");}
            else {print!(".");}
        }
        println!("");
    }
    // println!("{:?}", locations);
}

fn main() {

    let start_time = Instant::now();

    let input = fs::read_to_string("src/input.txt")
        .expect("File was not readable");
    let input: Vec<_> = input.split('\n').collect();

    // let testing: Vec<_> = input.iter().map(|e| inst { Direction: *e[0], Count: e[1]}).collect();
    let mut insts: Vec<Inst> = vec![];
    for line in input {
        if line.is_empty() {break;}
        let components: Vec<_> = line.split(' ').collect();
        let direction = match components[0] {
            "R" => {Orientations::Right},
            "U" => {Orientations::Up},
            "L" => {Orientations::Left},
            _ => {Orientations::Down},
        };
        let count: u32 = components[1].parse()
            .expect("Number cannot be converted");
        insts.push(Inst {direction: direction, count: count});
    }


    let mut locations: HashSet<(i32, i32)> = HashSet::new();
    locations.insert((0,0)); // tail starts at 0,0

    let rope_length = 10;
    let mut single_rope: Vec<Position> = Vec::with_capacity(rope_length);
    for _ in 0..rope_length{
        single_rope.push(Position { x: 0, y: 0 });
    }
    let num_knots = single_rope.len();

    for inst in insts {
        let direction_moving = inst.direction;
        let count = inst.count;
        // println!("{:?}", inst);

        for _ in 0..count {
            
            // move head

            single_rope[0].x += if direction_moving == Orientations::Right {1} 
                else if direction_moving == Orientations::Left {-1} 
                else {0};
            single_rope[0].y += if direction_moving == Orientations::Up {1} 
                else if direction_moving == Orientations::Down {-1}
                else {0};


            if distance_t2(&single_rope[0], &single_rope[1]) < 2.0 {continue;}

            for x in 0..single_rope.len()-1 {
                let top_knot = single_rope[x];
                let bot_knot = &mut single_rope[x+1];

                if distance_t2(&top_knot, &bot_knot) < 2.0 {continue;}
                
                let move_command = next_move_t2(&top_knot, bot_knot);
                bot_knot.x += move_command.0;
                bot_knot.y += move_command.1;
                if x == num_knots-2 {locations.insert((bot_knot.x, bot_knot.y));}
            }  
        }
        // print_board(&single_rope);
        // println!("Tail was in {:?} locations", locations.len());

    }
    println!("Tail was in {:?} locations", locations.len());
    println!("Elapsed time: {:?}", Instant::now() - start_time);

    
}
