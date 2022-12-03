use std::fs;

fn get_priority(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 'a' as i32 + 1,
        'A'..='Z' => c as i32 - 'A' as i32 + 27,
        _ => 0,
    }
}

fn get_common_letter(left: &str, right: &str) -> Option<char> {
    right.chars().find(|c| left.contains(*c))
}

fn get_badge(one: &str, two: &str, three: &str) -> Option<char> {
    three.chars().find(|c| one.contains(*c) && two.contains(*c))
}

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Could not open file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let priority: i32 = lines.iter()
        .filter_map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            get_common_letter(left, right)
        })
        .map(|c| get_priority(c))
        .sum();

    println!("Sum of priorities: {priority}");

    let badge_priority: i32 = lines.chunks(3)
        .filter_map(|group| {
            get_badge(group.get(0)?, group.get(1)?, group.get(2)?)
        })
        .map(|c| get_priority(c))
        .sum();

    println!("Sum of badge priorities: {badge_priority}");
}
