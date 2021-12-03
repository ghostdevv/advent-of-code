use std::fs;
use std::isize;

fn main() {
    let input = fs::read_to_string("src/input.txt")
                    .expect("File died");

    let lines = input.lines();

    let mut line_total: [i32;12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut line_count = 0;

    for line in lines {
        let mut i = 0;

        for char in line.chars() {
            if char == '1' {
                line_total[i] += 1;
            } 
            
            i += 1;
        }

        line_count += 1;
    }

    let mut gamma: String = String::from("");
    let mut epsilon: String = String::from("");

    for x in line_total {
        let mostly_one: bool = x > line_count / 2;

        match mostly_one {
            true => { gamma.push('1'); epsilon.push('0'); }
            false => { gamma.push('0'); epsilon.push('1'); }
        }
    }

    println!("Gamma: {} , Epsilon: {}", gamma, epsilon);

    let g_bin = isize::from_str_radix(&gamma, 2).unwrap();
    let e_bin = isize::from_str_radix(&epsilon, 2).unwrap();

    println!("Res: {}", g_bin * e_bin);
}
