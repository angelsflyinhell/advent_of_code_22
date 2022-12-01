use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").expect("File does not exist.");

    let mut calories = 0;
    let mut elves = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            elves.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<i32>().expect("Could not parse string to integer uwu");
        }
    }

    elves.sort_by(|a, b| b.cmp(a));

    println!("{:?}", elves[0]);

    let sum = elves[0] + elves[1] + elves[2];

    print!("{:?}", sum);
}
