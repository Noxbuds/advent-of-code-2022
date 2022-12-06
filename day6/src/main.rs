use std::{fs, collections::HashSet};

fn is_unique(slice: &str) -> bool {
    let mut set = HashSet::new();
    let mut unique = true;

    for c in slice.chars() {
        // insert returns true if it was inserted, false if it was there already,
        // so if any are duplicated it'll be false and && will be false
        unique &= set.insert(c);
    }
    
    return unique;
}

fn get_marker_end(string: &str, width: usize) -> Option<usize> {
    for i in 0..string.len() - width {
        let substr = &string[i..i+width];
        let unique = is_unique(substr);

        if unique {
            return Some(i + width);
        }
    }
    return None;
}

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Could not open file");

    if let Some(id) = get_marker_end(&contents, 4) {
        println!("Packet marker ends at {id}");
    }

    if let Some(id) = get_marker_end(&contents, 14) {
        println!("Messagn marker ends at {id}");
    }
}
