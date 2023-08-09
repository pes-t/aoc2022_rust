use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt")
        .expect("File reading failed");
    let input: Vec<&str> = input.split('\n').collect();

    let mut moves: Vec<(char, char)> = vec![];
    for line in &input {
        if line.is_empty() { continue };
        let a: Vec<char> = line.chars().collect();
        moves.push((a[0], a[2]));
    }

    let a: Vec<_> = input.iter().map(|x| x.chars().collect::<Vec<_>>()).collect();   // works but not in tuple form yet 

    // println!("{:?}", input);
    // println!("{:?}", moves);
    // println!("{:?}", a);

    let mut score = 0;

    for round in &moves {
        let opponent_move = round.0;
        let player_move = round.1;

        // A = Rock, B = Paper, C = Scissors
        // X = Rock, Y = Paper, Z = Scissors
        match opponent_move {
            'A' => { // Rock
                match player_move {
                    'X' => score += 3, 
                    'Y' => score += 6,
                    // 'Z' => score += 0,
                    _ => score += 0
                }
            },
            'B' => {
                match player_move {
                    'X' => score += 0,
                    'Y' => score += 3,
                    'Z' => score += 6,
                    _ => score += 0
                }
            },
            'C' => {
                match player_move {
                    'X' => score += 6,
                    'Y' => score += 0,
                    'Z' => score += 3,
                    _ => score += 0
                }
            },
            _ => {
                score += 0;
            }
        }

        match player_move {
            'X' => score += 1,
            'Y' => score += 2,
            'Z' => score += 3,
            _ => score += 0
        }
    }

    println!("part 1 - {:?}", score);

    let mut p2_score = 0;
    for round in &moves {
        let outcome = round.1;
        let opponent_move = round.0;

        match outcome {
            'X' => { //Lose the round
                match opponent_move {
                    'A' => p2_score += 3, // need to choose scissors
                    'B' => p2_score += 1,
                    'C' => p2_score += 2,
                    _ => p2_score += 0
                }
            },
            'Y' => { //Draw the round
                p2_score += 3;
                match opponent_move {
                    'A' => p2_score += 1, // need to choose scissors
                    'B' => p2_score += 2,
                    'C' => p2_score += 3,
                    _ => p2_score += 0
                }   
            },
            'Z' => { //Win the round
                p2_score += 6;
                match opponent_move {
                    'A' => p2_score += 2, // need to choose scissors
                    'B' => p2_score += 3,
                    'C' => p2_score += 1,
                    _ => p2_score += 0
                } 
            },
            _ => p2_score += 0
        }
    }
    
    println!("part 2 - {:?}", p2_score);





}
