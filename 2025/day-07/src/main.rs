use std::collections::HashMap;
use std::fmt::Display;

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug, Clone)]
enum Square {
    Space,
    Splitter,
    Start,
    Beam,
}

impl TryFrom<char> for Square {
    type Error = &'static str;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        match char {
            '.' => Ok(Square::Space),
            '^' => Ok(Square::Splitter),
            '|' => Ok(Square::Beam),
            'S' => Ok(Square::Start),
            _ => Err("Unknown char {char}"),
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Square::Space => ".",
            Square::Splitter => "^",
            Square::Start => "S",
            Square::Beam => "|",
        };

        write!(f, "{}", str)
    }
}

#[derive(Clone)]
struct Row(Vec<Square>);

impl Row {
    fn iter<'a>(&'a self) -> std::slice::Iter<'a, Square> {
        self.0.iter()
    }

    fn square_at(&self, pos: usize) -> Option<&Square> {
        if &self.squares().len() <= &pos {
            None
        } else {
            Some(&self.squares()[pos])
        }
    }

    fn square_at_mut(&mut self, pos: usize) -> Option<&mut Square> {
        if &self.squares().len() <= &pos {
            None
        } else {
            Some(&mut self.squares_mut()[pos])
        }
    }

    fn squares(&self) -> &Vec<Square> {
        &self.0
    }

    fn squares_mut(&mut self) -> &mut Vec<Square> {
        &mut self.0
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for square in &self.0 {
            write!(f, "{}", square)?;
        }

        Ok(())
    }
}

impl FromIterator<Square> for Row {
    fn from_iter<T: IntoIterator<Item = Square>>(iter: T) -> Self {
        Row(iter.into_iter().collect::<Vec<_>>())
    }
}

#[derive(Clone)]
struct Grid(Vec<Row>);

impl Grid {
    fn rows(&self) -> &Vec<Row> {
        &self.0
    }

    fn rows_mut(&mut self) -> &mut Vec<Row> {
        &mut self.0
    }

    fn start_pos(&self) -> Option<(usize, usize)> {
        for (row_i, row) in self.rows().iter().enumerate() {
            for (col_i, square) in row.iter().enumerate() {
                if let Square::Start = square {
                    if row_i + 1 < self.rows().len() {
                        return Some((row_i + 1, col_i));
                    }
                }
            }
        }

        None
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows() {
            write!(f, "{}", row)?;
        }

        Ok(())
    }
}

impl FromIterator<Row> for Grid {
    fn from_iter<T: IntoIterator<Item = Row>>(iter: T) -> Self {
        Grid(iter.into_iter().collect())
    }
}

fn trace_beam(rows: &mut Vec<Row>, row_i: usize, col_i: usize) {
    if row_i >= rows.len() {
        return;
    }

    let square = match rows[row_i].square_at(col_i) {
        Some(s) => s,
        None => return,
    };

    match square {
        Square::Space => {
            if let Some(square) = rows[row_i].square_at_mut(col_i) {
                *square = Square::Beam;
            }

            trace_beam(rows, row_i + 1, col_i);
        }
        Square::Beam => {
            // trace_beam(rows, row_i + 1, col_i);
        }
        Square::Splitter => {
            // Left
            if col_i > 0 {
                trace_beam(rows, row_i, col_i - 1);
            }

            // Right
            if col_i + 1 < rows[row_i].squares().len() {
                trace_beam(rows, row_i, col_i + 1);
            }
        }
        _ => panic!("unexpected square in trace_beam {square}"),
    }
}

fn count_paths(
    rows: &Vec<Row>,
    row_i: usize,
    col_i: usize,
    paths: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(&count) = paths.get(&(row_i, col_i)) {
        return count;
    }

    if row_i >= rows.len() {
        return 1;
    }

    let square = match rows[row_i].square_at(col_i) {
        Some(s) => s,
        None => return 0,
    };

    let result = match square {
        Square::Space | Square::Beam | Square::Start => {
            // Continue straight down
            count_paths(rows, row_i + 1, col_i, paths)
        }
        Square::Splitter => {
            let mut total = 0;

            // Count paths going left
            if col_i > 0 {
                total += count_paths(rows, row_i, col_i - 1, paths);
            }

            // Count paths going right
            if col_i + 1 < rows[row_i].squares().len() {
                total += count_paths(rows, row_i, col_i + 1, paths);
            }

            total
        }
    };

    // Cache the result
    paths.insert((row_i, col_i), result);
    result
}

fn part_one(mut grid: Grid) -> i32 {
    let start_pos = grid.start_pos().expect("unable to find start position");
    trace_beam(grid.rows_mut(), start_pos.0, start_pos.1);

    grid.rows().iter().enumerate().fold(0, |mut total, (row_i, row)| {
        if &grid.rows().len() <= &(row_i + 1) {
            return total;
        }

        let next_row = &grid.rows()[row_i + 1];

        for (square_i, square) in row.squares().iter().enumerate() {
            if let Square::Beam = square
                && let Some(Square::Splitter) = next_row.square_at(square_i)
            {
                total += 1;
            }
        }

        total
    })
}

fn part_two(grid: Grid) -> usize {
    let start_pos = grid.start_pos().expect("unable to find start position");
    let mut paths = HashMap::new();
    count_paths(grid.rows(), start_pos.0, start_pos.1, &mut paths)
}

fn main() {
    let grid = INPUT
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| Square::try_from(char).unwrap())
                .collect::<Row>()
        })
        .collect::<Grid>();

    println!("Part One: {}", part_one(grid.clone()));
    println!("Part Two: {}", part_two(grid.clone()));
    println!("\n=======================");
}
