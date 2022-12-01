use std::fs;

// Still learning rust so bear with me XD

fn main() {
    let contents = fs::read_to_string("src/input.txt").expect("Unable to read input");

    let mut item_groups: Vec<Vec<String>> = vec![];
    let mut current_group: Vec<String> = vec![];

    for item in contents.lines() {
        if item.trim() == "" {
            item_groups.push(current_group.clone());
            current_group = vec![];
        } else {
            current_group.push(item.to_string());
        }
    }

    let mut calorie_counts: Vec<u32> = vec![];

    for group in item_groups {
        let mut count: u32 = 0;

        for item in group {
            count += item.parse::<u32>().unwrap();
        }

        calorie_counts.push(count);
    }

    calorie_counts.sort();

    let mut top_three = calorie_counts.iter().rev().take(3).peekable();

    println!("Highest calorie count is: {:?}", top_three.peek());

    println!(
        "Sum of top three calorie counts is: {:?}",
        top_three.sum::<u32>()
    )
}
