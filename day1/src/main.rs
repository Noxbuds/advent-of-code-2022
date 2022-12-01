use std::fs;

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).unwrap();
    let lines = contents.split("\n");
    
    let mut sum = 0;
    let mut totals: Vec<i32> = vec![];

    for line in lines {
        if let Ok(value) = line.trim_end().parse::<i32>() {
            sum += value;
        } else {
            totals.push(sum);
            sum = 0;
        }
    }

    totals.sort();
    totals.reverse();

    let top_count = 3;
    let top_total = totals.iter().take(top_count).fold(0, |a, b| a + *b);

    println!("The top {top_count} elves are carrying {top_total} calories");
}
