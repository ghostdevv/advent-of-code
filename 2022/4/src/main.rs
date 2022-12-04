use std::{collections::HashSet, num::ParseIntError};

fn parse_elf_section(line: &str) -> Result<HashSet<i32>, ParseIntError> {
    let (start, end) = line.split_once("-").unwrap();

    let num_one = start.parse::<i32>()?;
    let num_two = end.parse::<i32>()?;

    let mut set = HashSet::new();

    for num in num_one..=num_two {
        set.insert(num);
    }

    Ok(set)
}

fn part_one() {
    let input = include_str!("input.txt");

    let mut sum = 0;

    for line in input.lines() {
        let (elf_one_str, elf_two_str) = line.split_once(",").unwrap();

        let elf_one = parse_elf_section(elf_one_str).unwrap();
        let elf_two = parse_elf_section(elf_two_str).unwrap();

        if elf_one.is_subset(&elf_two) || elf_two.is_subset(&elf_one) {
            sum += 1;
        }
    }

    println!("Sum: {sum}");
}

fn part_two() {
    let input = include_str!("input.txt");

    let mut sum = 0;

    for line in input.lines() {
        let (elf_one_str, elf_two_str) = line.split_once(",").unwrap();

        let elf_one = parse_elf_section(elf_one_str).unwrap();
        let elf_two = parse_elf_section(elf_two_str).unwrap();

        let intersects = elf_one.intersection(&elf_two);

        if intersects.count() > 0 {
            sum += 1;
        }
    }

    println!("Sum: {sum}");
}

fn main() {
    part_one();
    part_two();
}
