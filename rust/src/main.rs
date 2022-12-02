use std::{fs};
 // doesnt really work lmao 
fn main() {
    let mut score = 0;

    let input = fs::read_to_string("./input.txt").expect("File does not exist.");

    for line in input.lines() {
        let mut i = 0;
        let moves: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();

        let opponent = "ABC".find(&moves[0]).unwrap() as i32;
        let player = "XYZ".find(&moves[1]).unwrap() as i32;
        let result = player - opponent;

        score += player + 1;
        
        println!("{} {}", i, result % 3);

        match result % 3 {
            1 | -2 => score += 6,
            0 => score += 3,
            -1 | 2 => score += 0,
            _ => print!("")
        }
        i += 1;
    }
    println!("{:?}", score);
}