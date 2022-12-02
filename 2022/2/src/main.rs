use std::fs;

fn resolve_choice(choice: &str) -> i64 {
    match choice {
        "A" => 1,
        "B" => 2,
        "C" => 3,

        "X" => 1,
        "Y" => 2,
        "Z" => 3,

        _ => todo!(),
    }
}

fn part_one() {
    let contents = fs::read_to_string("src/input.txt").expect("Unable to read input text");

    let mut points = 0;

    for line in contents.lines() {
        let (opponent_letter, letter) = line.split_once(" ").unwrap();

        let opponent_cscore = resolve_choice(opponent_letter);
        let cscore = resolve_choice(letter);

        // Draw
        if opponent_cscore == cscore {
            points += cscore + 3;
            continue;
        }

        let should_win = match opponent_cscore {
            // If opponent has rock you need paper
            1 => cscore == 2,

            // If opponent has paper you need scissors
            2 => cscore == 3,

            // If opponent has scissors you need rock
            3 => cscore == 1,
            _ => todo!(),
        };

        points += cscore + if should_win { 6 } else { 0 };
    }

    println!("Your points are: {}", points);
}

fn part_two() {
    let contents = fs::read_to_string("src/input.txt").expect("Unable to read input text");

    let mut points = 0;

    for line in contents.lines() {
        let (opponent_letter, letter) = line.split_once(" ").unwrap();

        let opponent_cscore = resolve_choice(opponent_letter);

        // Draw
        if letter == "Y" {
            points += opponent_cscore + 3;
            continue;
        }

        if letter == "X" {
            // Lose

            let cscore = match opponent_cscore {
                // If opponent has rock you need scissors
                1 => 3,

                // If opponent has paper you need rock
                2 => 1,

                // If opponent has scissors you need paper
                3 => 2,

                _ => todo!(),
            };

            points += cscore;
        } else if letter == "Y" {
            // Draw

            points += opponent_cscore + 3;
        } else {
            // Win

            let cscore = match opponent_cscore {
                // If opponent has rock you need paper
                1 => 2,

                // If opponent has paper you need scissors
                2 => 3,

                // If opponent has scissors you need rock
                3 => 1,

                _ => todo!(),
            };

            points += cscore + 6;
        }
    }

    println!("Your points are: {}", points);
}

fn main() {
    println!("Part One:");
    part_one();

    println!("Part Two:");
    part_two();
}
