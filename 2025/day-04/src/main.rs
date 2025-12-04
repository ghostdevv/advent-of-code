const INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
enum Space {
    Paper,
    Empty,
}

struct Grid {
    grid: Vec<Vec<Space>>,
}

impl Grid {
    fn from_str(input: &str) -> Grid {
        let grid = input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '@' => Space::Paper,
                        '.' => Space::Empty,
                        _ => panic!("unknown char '{}' in \"{}\"", c, line),
                    })
                    .collect()
            })
            .collect();

        Grid { grid }
    }

    fn get_with_neg(&self, x: i32, y: i32) -> Option<&Space> {
        if x < 0 || y < 0 {
            return None;
        }

        self.get(x as usize, y as usize)
    }

    fn get(&self, x: usize, y: usize) -> Option<&Space> {
        if y >= self.grid.len() {
            return None;
        }

        let inner = &self.grid[y];

        if x >= inner.len() {
            return None;
        }

        Some(&inner[x])
    }

    fn next(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if x + 1 < self.grid[y].len() {
            return Some((x + 1, y));
        }

        if y + 1 < self.grid.len() {
            return Some((0, y + 1));
        }

        None
    }
}

struct GridIntoIterator<'a> {
    grid: &'a Grid,
    next: Option<(usize, usize)>,
}

impl<'a> IntoIterator for &'a Grid {
    type Item = (usize, usize, &'a Space);
    type IntoIter = GridIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        GridIntoIterator {
            grid: self,
            next: Some((0, 0)),
        }
    }
}

impl<'a> Iterator for GridIntoIterator<'a> {
    type Item = (usize, usize, &'a Space);

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => None,
            Some((x, y)) => match self.grid.get(x, y) {
                None => None,
                Some(space) => {
                    self.next = self.grid.next(x, y);
                    Some((x, y, space))
                }
            },
        }
    }
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1), // top left
    (0, -1),  // top
    (1, -1),  // top right
    (1, 0),   // right
    (1, 1),   // bottom right
    (0, 1),   // bottom
    (-1, 1),  // bottom left
    (-1, 0),  // left
];

fn main() {
    let grid = Grid::from_str(INPUT);
    let mut accessible = 0;

    for (x, y, space) in grid.into_iter() {
        if x == 0 && y != 0 {
            print!("\n")
        }

        match space {
            Space::Empty => print!("."),
            Space::Paper => {
                let mut conflicts = 0;
                for direction in DIRECTIONS {
                    let dx = (x as i32) + direction.0;
                    let dy = (y as i32) + direction.1;

                    if let Some(space) = grid.get_with_neg(dx, dy) {
                        match space {
                            Space::Paper => conflicts += 1,
                            _ => {}
                        }
                    }

                    if conflicts >= 4 {
                        break;
                    }
                }

                print!("{}", if conflicts < 4 { accessible += 1; "x" } else { "@" })
            }
        }
    }

    print!("\n\n");
    println!("=======================");
    println!("Part One: {}", accessible);
}
