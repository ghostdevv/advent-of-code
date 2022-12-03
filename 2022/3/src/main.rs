fn char_to_value(c: char) -> usize {
    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let uppercase = lowercase.to_uppercase();

    match lowercase.find(c) {
        Some(x) => return x + 1,
        None => {}
    }

    match uppercase.find(c) {
        Some(x) => return x + 27,
        None => {}
    }

    todo!();
}

fn part_one() {
    let input = include_str!("input.txt");

    // Sum of priorities
    let mut sum = 0;

    // For each backpack
    for line in input.lines() {
        // Get the first and second compartment
        let (one, two) = line.split_at(line.len() / 2);

        // Loop over all chars in first compartment
        for c in one.chars() {
            // If the second compartment contains that char add it's value to the sum and exit
            if two.contains(c) {
                sum += char_to_value(c);
                break;
            }
        }
    }

    println!("Sum of priorities: {sum}")
}

fn part_two() {
    let input = include_str!("input.txt");
    let lines = input.lines().collect::<Vec<&str>>();

    // Sum of priorities
    let mut sum = 0;

    // Chunk the lines into groups of three
    for chunk in lines.chunks(3) {
        // Get the first, second, and third backpack
        if let [one, two, three] = chunk {
            // Loop over all the chars in the first backpack
            for c in one.chars() {
                // If the second and third backpack has that char add it's value to the total and exit
                if two.contains(c) && three.contains(c) {
                    sum += char_to_value(c);
                    break;
                }
            }
        }
    }

    println!("Sum of priorities: {sum}");
}

fn main() {
    part_one();
    part_two();
}
