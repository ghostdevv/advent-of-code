use std::fmt;

const INPUT: &str = include_str!("./input.txt");

enum Rotation {
    Left(i32),
    Right(i32),
}

impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Left(num) => write!(f, "L{}", num),
            Self::Right(num) => write!(f, "R{}", num),
        }
    }
}

fn main() {
    let rotations = INPUT
        .lines()
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (direction, num) = line.split_at(1);
            let num =
                i32::from_str_radix(num, 10).expect(&format!("failed to parse line: {}", line));

            match direction {
                "L" => Rotation::Left(num),
                "R" => Rotation::Right(num),
                _ => panic!("invalid direction: {}", direction),
            }
        })
        .collect::<Vec<Rotation>>();

    let mut position = 50;

    let mut part_one_zeros = 0;
    let mut part_two_zeros = 0;

    for rotation in rotations {
        match rotation {
            Rotation::Left(degrees) => {
                for _ in 0..degrees {
                    position = if position == 0 { 99 } else { position - 1 };

                    if position == 0 {
                        part_two_zeros += 1;
                    }
                }
            }
            Rotation::Right(degrees) => {
                for _ in 0..degrees {
                    position = if position == 99 { 0 } else { position + 1 };

                    if position == 0 {
                        part_two_zeros += 1;
                    }
                }
            }
        }

        if position == 0 {
            part_one_zeros += 1;
        }

        println!("{} {}", rotation, position)
    }

    println!("\n=============================");
    println!("Position is {}", position);
    println!("Part One: {}", part_one_zeros);
    println!("Part Two: {}", part_two_zeros);
}
