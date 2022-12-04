use std::{fs, ops::RangeInclusive};

type Bound = RangeInclusive<i32>;

fn get_range(text: &str) -> Option<Bound> {
    let (lower_str, upper_str) = text.split_once('-')?;
    let lower = lower_str.parse::<i32>();
    let upper = upper_str.parse::<i32>();

    if let (Ok(lower), Ok(upper)) = (lower, upper) {
        Some(lower..=upper)
    } else {
        None
    }
}

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Could not open file");
    let lines = contents.split("\n");

    let bounds: Vec<(Bound, Bound)> = lines
        .filter_map(|line| {
            let (left, right) = line.split_once(',')?;
            let left_bounds = get_range(left)?;
            let right_bounds = get_range(right)?;

            Some((left_bounds, right_bounds))
        })
        .collect();

    let complete_overlap = bounds.iter()
        .filter(|(left, right)| {
            let right_in_left = left.contains(&right.start()) && left.contains(&right.end());
            let left_in_right = right.contains(&left.start()) && right.contains(&left.end());

            right_in_left || left_in_right
        })
        .count();

    println!("Number completely overlapping: {complete_overlap}");

    let overlapping = bounds.iter()
        .filter(|(left, right)| {
            left.contains(&right.start()) || left.contains(&right.end()) || right.contains(&left.start()) || right.contains(&left.end())
        })
        .count();       

    println!("Number overlapping: {overlapping}");
}
