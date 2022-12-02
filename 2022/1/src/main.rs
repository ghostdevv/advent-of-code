use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("Unable to read input");

    let mut calorie_counts = contents
        // Replace all \n with a comma
        .replace("\n", ",")
        // Because there is a \n\n to group numbers there will be a double comma in places
        .split(",,")
        // We can split by that comma to get each group, split that into a vec, and then get the sum of the vec
        .map(|item| item.split(",").map(|n| n.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();

    // Sort from highest to lowest
    calorie_counts.sort_by(|a, b| b.cmp(a));

    println!("Highest Calorie Count: {:?}", calorie_counts.first());

    println!(
        "Sum of top three calorie counts: {:?}",
        calorie_counts.iter().take(3).sum::<u32>()
    );
}
