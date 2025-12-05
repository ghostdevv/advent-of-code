use std::ops::RangeInclusive;

const INPUT: &str = include_str!("./input.txt");

trait RangeMerging {
    fn merge_to_disjoint(self) -> Vec<RangeInclusive<i64>>;
}

impl RangeMerging for Vec<RangeInclusive<i64>> {
    /// Merge ranges to non-overlapping (disjoint) ranges
    fn merge_to_disjoint(mut self) -> Vec<RangeInclusive<i64>> {
        self.sort_by_key(|range| *range.start());

        let mut new_ranges = Vec::<RangeInclusive<i64>>::new();
        let mut current = self[0].to_owned();

        for range in self.into_iter().skip(1) {
            if current.end() < range.start() {
                new_ranges.push(current);
                current = range;
                continue;
            }

            if current.end() >= range.start() {
                let new_end = if current.end() > range.end() {
                    current.end()
                } else {
                    range.end()
                };

                current = *current.start()..=*new_end;
            }
        }

        new_ranges.push(current.to_owned());

        return new_ranges;
    }
}

fn main() {
    let (fresh, available) = INPUT
        .split_once("\n\n")
        .expect("unable to find first/second half of list");

    let fresh = fresh
        .trim()
        .lines()
        .map(|line| {
            let (start, end) = line
                .split_once("-")
                .expect(&format!("Unable to split fresh line: \"{}\"", line));

            let start = i64::from_str_radix(start, 10).expect(&format!(
                "Unable to parse fresh start to number \"{}\"",
                start
            ));

            let end = i64::from_str_radix(end, 10)
                .expect(&format!("Unable to parse fresh end to number \"{}\"", end));

            start..=end
        })
        .collect::<Vec<_>>()
        .merge_to_disjoint();

    let available = available
        .trim()
        .lines()
        .map(|line| {
            i64::from_str_radix(line, 10).expect(&format!(
                "Unable to parse available to number: \"{}\"",
                line
            ))
        })
        .collect::<Vec<_>>();

    let mut total_fresh = 0;

    for available in available.iter() {
        for fresh_range in fresh.iter() {
            if fresh_range.contains(&available) {
                total_fresh += 1;
                break;
            }
        }
    }

    let total_possible_fresh_ids = fresh
        .iter()
        .fold(0, |total, range| total + (range.end() - range.start() + 1));

    println!("Part One: {}", total_fresh);
    println!("Part Two: {}", total_possible_fresh_ids);
}
