use std::{fs, collections::HashSet};

fn get_direction(line: &str) -> Option<((i32, i32), i32)> {
    let (direction, amount_str) = line.split_once(' ')?;

    let unit_vector = match direction {
        "U" => Some((0, 1)),
        "D" => Some((0, -1)),
        "L" => Some((-1, 0)),
        "R" => Some((1, 0)),
        _ => None,
    }?;

    let amount = amount_str.parse::<i32>();
    if let Ok(amount) = amount {
        Some((unit_vector, amount))
    } else {
        None
    }
}

fn get_new_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;

    let dist = dx * dx + dy * dy;

    // only move if two or more away, with square dist that's 4
    if dist >= 4 {
        (tail.0 + dx.clamp(-1, 1), tail.1 + dy.clamp(-1, 1))
    } else {
        tail
    }
}

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Could not open file");
    let lines = contents.split('\n');

    let mut rope: Vec<(i32, i32)> = (0..10).map(|_| (0, 0)).collect();
    let mut visited = HashSet::new();

    for line in lines {
        let vector = get_direction(line);

        if let Some(((dx, dy), amount)) = vector {
            for _ in 0..amount {
                rope[0].0 += dx;
                rope[0].1 += dy;

                for i in 1..rope.len() {
                    rope[i] = get_new_tail(rope[i - 1], rope[i]);
                }

                if let Some(tail) = rope.last() {
                    visited.insert(tail.clone());
                }
            }
        }

    }

    let num_unique = visited.len();
    println!("Number of unique locations: {num_unique}");
}
