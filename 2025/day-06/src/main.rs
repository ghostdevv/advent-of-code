const INPUT: &str = include_str!("./input.txt");

#[derive(Debug, Clone)]
struct Column<'a> {
    operation: Operation,
    content: Vec<Vec<&'a str>>,
}

#[derive(Debug, Clone)]
enum Operation {
    Multiply,
    Add,
}

#[derive(Debug, Clone)]
struct Sum {
    operation: Operation,
    nums: Vec<i64>,
}

impl std::iter::Sum<Sum> for i64 {
    fn sum<I: Iterator<Item = Sum>>(iter: I) -> Self {
        iter.filter(|sum| sum.nums.len() > 0)
            .map(|sum| match sum.operation {
                Operation::Add => sum.nums.iter().sum(),
                Operation::Multiply => sum.nums.iter().fold(1, |total, n| total * n),
            })
            .sum::<Self>()
    }
}

fn part_one(columns: Vec<Column>) -> i64 {
    columns
        .into_iter()
        .map(|column| Sum {
            operation: column.operation,
            nums: column
                .content
                .into_iter()
                .map(|num_parts| {
                    num_parts
                        .join("")
                        .trim()
                        .parse::<i64>()
                        .unwrap_or_else(|_| panic!("Failed to parse num \"{:?}\"", num_parts))
                })
                .collect::<Vec<_>>(),
        })
        .sum()
}

fn part_two(columns: Vec<Column>) -> i64 {
    columns
        .into_iter()
        .map(|column| Sum {
            operation: column.operation,
            nums: (0..column.content[0].len())
                .map(|num_part_idx| {
                    let num_str = column
                        .content
                        .iter()
                        .map(|row| row[num_part_idx])
                        .collect::<Vec<_>>()
                        .join("")
                        .trim()
                        .to_string();

                    num_str
                        .parse::<i64>()
                        .unwrap_or_else(|_| panic!("Failed to parse num \"{}\"", num_str))
                })
                .collect::<Vec<_>>(),
        })
        .sum()
}

fn main() {
    let (input, operations_line) = INPUT.rsplit_once("\n").expect("failed to split input");

    let input = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut operations_indexed = operations_line
        .chars()
        .enumerate()
        .filter_map(|(i, char)| {
            if char == ' ' {
                return None;
            }

            let operation = match char {
                '*' => Operation::Multiply,
                '+' => Operation::Add,
                _ => panic!("Invalid operation \"{}\"", char),
            };

            Some((operation, i))
        })
        .peekable();

    let columns = std::iter::from_fn(|| {
        let (operation, start) = operations_indexed.next()?;
        let next = operations_indexed.peek();

        let content = input
            .iter()
            .map(|line| {
                let end = next
                    .map(|(_, i)| (*i - 1).min(line.len()))
                    .unwrap_or(line.len());

                line[start..end]
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Some(Column { operation, content })
    })
    .collect::<Vec<_>>();

    println!("===================");
    println!("Part One: {}", part_one(columns.clone()));
    println!("Part Two: {}", part_two(columns));
}
