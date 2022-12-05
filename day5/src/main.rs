use std::fs;

fn get_stacks(lines: Vec<&str>) -> Vec<Vec<&str>> {
    let stack_width = 4;
    let mut stacks: Vec<Vec<&str>> = vec![];

    for line in lines {
        for i in 0..=line.len() / stack_width {
            let index = i * stack_width + 1;
            let chr = &line[index..index+1];

            if stacks.len() <= i {
                stacks.push(vec![]);
            }

            if chr != " " {
                stacks[i].insert(0, chr.clone());
            }
        }
    }

    return stacks;
}

type Move = (i32, usize, usize);

fn get_move(line: &str) -> Option<Move> {
    let (count_str, indices) = line
        .strip_prefix("move ")
        .and_then(|s| s.split_once(" from "))?;

    let (from_str, to_str) = indices.split_once(" to ")?;

    let count = count_str.parse();
    let from = from_str.parse();
    let to = to_str.parse();

    if let (Ok(count), Ok(from), Ok(to)) = (count, from, to) {
        Some((count, from, to))
    } else {
        None
    }
}

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Could not open file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let crate_lines = lines.iter()
        .filter(|line| line.contains("["))
        .map(|line| line.clone())
        .collect();

    let mut stacks = get_stacks(crate_lines);
    let moves = lines.iter().filter_map(|line| get_move(line));

    for (count, from, to) in moves {
        for i in 0..count {
            let x = stacks[from - 1].pop();
            let id = stacks[to - 1].len() - i as usize;
            if let Some(x) = x {
                stacks[to - 1].insert(id, x);
            }
        }
    }

    let top_values = stacks.into_iter()
        .filter_map(|stack| Some(stack.last()?.clone()))
        .collect::<Vec<&str>>()
        .join("");

    println!("The top crates for part 2 are {top_values}");
}
