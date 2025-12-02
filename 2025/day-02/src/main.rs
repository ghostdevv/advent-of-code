use color_eyre::{Result, eyre::eyre};

const INPUT: &str = include_str!("./input.txt");

fn id_has_one_rep(id: &str) -> bool {
    let len = id.len();

    if len % 2 != 0 {
        return false;
    }

    let (a, b) = id.split_at(len / 2);

    a == b
}

fn id_has_multiple_reps(id: &str) -> bool {
    let chars = id.chars().collect::<Vec<_>>();
    let len = chars.len();

    for i in 1..=len {
        // optimisation
        if len % i != 0 || len / i < 2 {
            continue;
        }

        let chunks = chars
            .chunks(i)
            .map(|chars| chars.iter().collect::<String>())
            .collect::<Vec<_>>();

        let first = &chunks[0];
        let equal = chunks.iter().all(|c| c == first);

        if equal {
            return true;
        }
    }

    false
}

fn main() {
    let pairs = INPUT
        .trim()
        .split(",")
        .map(|id_pair_str| -> Result<(i64, i64)> {
            let (left, right) = id_pair_str
                .split_once("-")
                .ok_or(eyre!("failed to split id pair: {}", id_pair_str))?;

            let left = i64::from_str_radix(left, 10)?;
            let right = i64::from_str_radix(right, 10)?;

            Ok((left, right))
        })
        .collect::<Vec<Result<(i64, i64)>>>();

    let mut part_one_total = 0;
    let mut part_two_total = 0;

    for pair in pairs {
        let (start, end) = pair.expect("invalid id");

        let mut p2 = Vec::<i64>::new();

        for id in start..=end {
            let id_str = id.to_string();

            if id_str.starts_with("0") || id_has_one_rep(&id_str) {
                part_one_total += id;
            }

            if id_str.starts_with("0") || id_has_multiple_reps(&id_str) {
                p2.push(id);
                part_two_total += id;
            }
        }

        if p2.len() > 0 {
            println!("invalid ids: {}-{} {:?}", start, end, p2);
        }
    }

    println!("\n====================");
    println!("Part one: {}", part_one_total);
    println!("Part two: {}", part_two_total);
}
