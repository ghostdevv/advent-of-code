use std::fs;

fn main() {
    let text = fs::read_to_string("src/input.txt")
        .expect("File died");

    let mut horizontal = 0;
    let mut depth = 0;

    let mut aim = 0;
    let mut p2_depth = 0;

    for line in text.lines() {
        let (instruction, value) = get_args(line);

        match instruction {
            x if x == "forward" => { horizontal += value; p2_depth += aim * value }
            x if x == "up" => { depth -= value; aim -= value }
            x if x == "down" => { depth += value; aim += value }

            _ => println!("Error in {}", instruction)
        }
    }

    println!("P1: {}", horizontal * depth);
    println!("P2: {}", horizontal * p2_depth);
}

fn get_args(line: &str) -> (&str, i32) {
    let args: Vec<&str> = line.split_whitespace()
                                .take(2)
                                .collect();

    return (args[0], args[1].parse::<i32>().unwrap())
}
