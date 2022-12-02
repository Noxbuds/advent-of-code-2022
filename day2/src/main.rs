use std::fs;

#[derive(PartialEq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

type ShapePair = (Shape, Shape);

enum RoundOutcome {
    Win,
    Draw,
    Lose,
}

fn get_shape(shape: &str) -> Option<Shape> {
    match shape {
        "A" | "X" => Some(Shape::Rock),
        "B" | "Y" => Some(Shape::Paper),
        "C" | "Z" => Some(Shape::Scissors),
        _ => None,
    }
}

fn get_outcome(string: &str) -> Option<RoundOutcome> {
    match string {
        "X" => Some(RoundOutcome::Lose),
        "Y" => Some(RoundOutcome::Draw),
        "Z" => Some(RoundOutcome::Win),
        _ => None,
    }
}

fn get_winning_shape(shape: &Shape) -> Shape {
    match shape {
        Shape::Scissors => Shape::Rock,
        Shape::Paper => Shape::Scissors,
        Shape::Rock => Shape::Paper,
    }
}

fn get_losing_shape(shape: &Shape) -> Shape {
    match shape {
        Shape::Rock => Shape::Scissors,
        Shape::Paper => Shape::Rock,
        Shape::Scissors => Shape::Paper,
    }
}

fn get_shape_score(shape: &Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn get_outcome_score(outcome: &RoundOutcome) -> i32 {
    match outcome {
        RoundOutcome::Win => 6,
        RoundOutcome::Draw => 3,
        RoundOutcome::Lose => 0,
    }
}

fn get_best_shape(opponent: &Shape, target_outcome: &RoundOutcome) -> Shape {
    match target_outcome {
        RoundOutcome::Win => get_winning_shape(&opponent),
        RoundOutcome::Draw => opponent.clone(),
        RoundOutcome::Lose => get_losing_shape(&opponent),
    }
}
    
fn get_pairings(lines: &Vec<&str>, use_target_outcome: bool) -> Vec<ShapePair> {
    lines.iter()
        .filter_map(|line| {
            let (opponent_str, player_str) = line.split_once(' ')?;

            let opponent = get_shape(opponent_str)?;
            let player = if use_target_outcome {
                get_best_shape(&opponent, &get_outcome(player_str)?)
            } else {
                get_shape(player_str)?
            };

            Some((opponent, player))
        })
        .collect()
}

fn get_score(pairings: Vec<ShapePair>) -> i32 {
    pairings.iter()
        .map(|(opponent, player)| {
            let outcome = if get_winning_shape(&opponent).eq(player) {
                RoundOutcome::Win
            } else if opponent == player {
                RoundOutcome::Draw
            } else {
                RoundOutcome::Lose
            };
            
            get_shape_score(&player) + get_outcome_score(&outcome)
        })
        .sum()
}

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Could not open file");
    let lines = contents.split("\n").collect();

    let p1_pairings = get_pairings(&lines, false);
    let p1_score = get_score(p1_pairings);

    let p2_pairings = get_pairings(&lines, true);
    let p2_score = get_score(p2_pairings);

    println!("Score with part 1 strategy: {p1_score}");
    println!("Score with part 2 strategy: {p2_score}");
}
