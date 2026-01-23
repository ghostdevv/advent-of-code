const INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
struct Shape<'a> {
    index: i32,
    shape: &'a str,
}

#[derive(Debug)]
struct Quantity<'a> {
    n: i32,
    shape: &'a Shape<'a>,
}

#[derive(Debug)]
struct Region<'a> {
    x: i32,
    y: i32,
    quantities: Vec<Quantity<'a>>,
}

fn area_simple(rect_area: i32, n_shapes: i32) -> bool {
    (3 * 3 * n_shapes) <= rect_area
}

// fn is_area_chopped(area: i32, region: &Region) -> bool {
//     area > (region
//         .quantities
//         .iter()
//         .map(|q| {
//             q.n * (q
//                 .shape
//                 .shape
//                 .chars()
//                 .filter(|c| *c == '#')
//                 .count() as i32)
//         })
//         .sum::<i32>())
// }

fn guess_region(region: &Region) -> bool {
    let area = region.x * region.y;
    let n_shapes = region.quantities.iter().map(|q| q.n).sum::<i32>();

    area_simple(area, n_shapes)

    // is_area_chopped(area, region)
}

fn main() {
    let lines = INPUT.trim().split("\n\n").collect::<Vec<_>>();
    let (shapes, regions) = lines.split_at(lines.len() - 1);

    let shapes = shapes
        .into_iter()
        .map(|str| {
            let (index, shape) = str
                .split_once("\n")
                .unwrap_or_else(|| panic!("failed to split shape str \"{str}\""));

            let index = index
                .strip_suffix(":")
                .expect("failed to remove shape index colon")
                .parse::<i32>()
                .expect("failed to parse index");

            Shape { index, shape }
        })
        .collect::<Vec<_>>();

    let regions = regions
        .into_iter()
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (area, quantities) = line
                .split_once(": ")
                .unwrap_or_else(|| panic!("failed to split region line \"{line}\""));

            let (x, y) = area
                .split_once("x")
                .unwrap_or_else(|| panic!("failed to split region line area \"{area}\""));

            let x = x
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("failed to parse region x \"{x}\""));

            let y = y
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("failed to parse region y \"{y}\""));

            let quantities = quantities
                .split(" ")
                .enumerate()
                .map(|(i, n)| Quantity {
                    n: n.parse::<i32>()
                        .unwrap_or_else(|_| panic!("failed to parse region quantity n \"{n}\"")),
                    shape: &shapes.get(i).unwrap(),
                })
                .collect::<Vec<_>>();

            Region { x, y, quantities }
        })
        .collect::<Vec<_>>();

    for region in regions.iter() {
        let fits = guess_region(region);

        println!(
            "{}x{} with {} shapes {}",
            region.x,
            region.y,
            region.quantities.iter().map(|q| q.n).sum::<i32>(),
            if fits { "✅️" } else { "❌️" }
        );
    }
}
