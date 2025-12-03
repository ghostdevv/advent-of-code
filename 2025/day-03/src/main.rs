const INPUT: &str = include_str!("./input.txt");

fn task(banks: &Vec<Vec<i32>>, flip_n: usize) -> i64 {
    banks
        .iter()
        .map(|bank| {
            let len = bank.len();

            let mut last_idx = 0;
            let mut joltage: i64 = 0;

            for n in 0..flip_n {
                let rn = flip_n - n - 1;
                let mut result: i64 = 0;

                for i in last_idx..(len - rn) {
                    let num = bank[i] as i64;

                    if result < num {
                        result = num;
                        last_idx = i + 1;
                    }
                }

                joltage += if rn == 0 {
                    result
                } else {
                    (10 as i64).pow(rn as u32) * result
                }
            }

            joltage
        })
        .fold(0, |total, bank| total + bank)
}

fn main() {
    let banks = INPUT
        .trim()
        .lines()
        .map(|line| {
            line.split("")
                .filter(|s| !s.is_empty())
                .map(|s| {
                    i32::from_str_radix(s, 10).expect(&format!("failed to parse {} to number", s))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("Part One: {}", task(&banks, 2));
    println!("Part Two: {}", task(&banks, 12));
}
