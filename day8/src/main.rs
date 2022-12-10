use std::{fs, vec};

type Heightmap = Vec<Vec<i32>>;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

// i is id in direction, j is id perpendicular to direction
fn get_id(direction: &Direction, map_size: usize, i: usize, j: usize) -> (usize, usize) {
    match direction {
        Direction::Left => (j, map_size - i - 1),
        Direction::Right => (j, i),
        Direction::Down => (map_size - i - 1, j),
        Direction::Up => (i, j),
    }
}

// travels through the map in a direction, pushing the max value along, almost like casting shadows
fn generate_max_map(map: &Heightmap, direction: Direction) -> Vec<Vec<bool>> {
    let mut visibility = vec![];

    let map_size = map.len();
    for y in 0..map_size {
        visibility.push(vec![]);
        for _ in 0..map_size {
            visibility[y].push(false);
        }
    }

    let mut max_heights = vec![];

    for i in 0..map_size {
        max_heights.push(0);

        for along in 0..map_size {
            let id = get_id(&direction, map_size, along, i);

            // max value at edges always equals the value there
            if along == 0 {
                visibility[id.0][id.1] = true;
                continue;
            }

            let prev_id = get_id(&direction, map_size, along - 1, i);
            let prev_map = map[prev_id.0][prev_id.1];
            max_heights[i] = prev_map.max(max_heights[i]);

            if map[id.0][id.1] > max_heights[i] {
                visibility[id.0][id.1] = true;
            }

            max_heights[i] = max_heights[i].max(map[id.0][id.1]);
        }
    }

    visibility
}

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Could not open file");
    let lines = contents.split('\n');

    // this was neater in my mind when I imagined it
    let map: Heightmap = lines.filter_map(|line| {
        let heights: Vec<i32> = line.split("")
            .filter_map(|c| {
                if let Ok(value) = c.parse() {
                    Some(value)
                } else {
                    None
                }
            })
            .collect();

        if heights.len() > 0 {
            Some(heights)
        } else {
            None
        }
    }).collect();

    let directions = [Direction::Left, Direction::Right, Direction::Up, Direction::Down];
    let visibility_maps = directions.map(|direction| generate_max_map(&map, direction));

    for (i, map) in visibility_maps.iter().enumerate() {
        println!("\n{:?}", directions[i]);
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                print!("{}", if map[y][x] { "#" } else { "-" });
            }
            println!("");
        }
    }

    let mut visibility: Vec<Vec<bool>> = vec![];
    println!("\nVisibility:");
    for y in 0..map.len() {
        visibility.push(vec![]);
        for x in 0..map[y].len() {
            let mut visible = false;
            for visibility_map in visibility_maps.iter() {
                visible |= visibility_map[y][x];
            }

            print!("{}", if visible { "#" } else { "-" });
            visibility[y].push(visible);
        }
        println!("");
    }

    let visible_count = visibility.into_iter()
        .flatten()
        .filter(|x| *x)
        .count();

    println!("The number of visible trees is {visible_count}");
}
