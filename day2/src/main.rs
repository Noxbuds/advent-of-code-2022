use std::{collections::HashMap, fs, env};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn get_shape(shape: &str) -> Option<Shape> {
    match shape {
        "A" | "X" => Some(Shape::Rock),
        "B" | "Y" => Some(Shape::Paper),
        "C" | "Z" => Some(Shape::Scissors),
        _ => None,
    }
}

type ShapePair = (Shape, Shape);

#[derive(Eq, PartialEq, Hash, Clone)]
enum RoundOutcome {
    Win,
    Draw,
    Lose,
}

fn get_outcome(string: &str) -> Option<RoundOutcome> {
    match string {
        "X" => Some(RoundOutcome::Lose),
        "Y" => Some(RoundOutcome::Draw),
        "Z" => Some(RoundOutcome::Win),
        _ => None,
    }
}

fn get_pairings_p1(lines: &Vec<&str>) -> Vec<ShapePair> {
    lines.iter()
        .filter_map(|line| {
            let (opponent_str, player_str) = line.split_once(' ')?;
            let opponent = get_shape(opponent_str)?;
            let player = get_shape(player_str)?;

            Some((opponent, player))
        })
        .collect()
}

fn get_pairings_p2(lines: &Vec<&str>, winning_pairs: &HashMap<Shape, Shape>, losing_pairs: &HashMap<Shape, Shape>) -> Vec<ShapePair> {
    lines.iter()
        .filter_map(|line| {
            let (opponent_str, player_str) = line.split_once(' ')?;

            let opponent = get_shape(opponent_str)?;
            let player = match get_outcome(player_str)? {
                RoundOutcome::Win => winning_pairs.get(&opponent),
                RoundOutcome::Draw => Some(&opponent),
                RoundOutcome::Lose => losing_pairs.get(&opponent),
            }?.clone();

            Some((opponent, player))
        })
        .collect()
}

fn get_score(pairings: Vec<ShapePair>, winning_pairs: &HashMap<Shape, Shape>, outcome_scores: &HashMap<RoundOutcome, i32>, shape_scores: &HashMap<Shape, i32>) -> i32 {
    let player_shape_score = pairings
        .iter()
        .filter_map(|(_, shape)| shape_scores.get(&shape))
        .fold(0, |a, b| a + b);

    let player_outcome_score = pairings
        .iter()
        .map(|(opponent, player)| {
            if winning_pairs.get(opponent) == Some(player) {
                RoundOutcome::Win
            } else if opponent == player {
                RoundOutcome::Draw
            } else {
                RoundOutcome::Lose
            }
        })
        .filter_map(|outcome| outcome_scores.get(&outcome))
        .fold(0, |a, b| a + b);

    return player_shape_score + player_outcome_score;
}

fn main() {
    // mapping of opponent choice -> winning choice
    let winning_pairs = HashMap::from([
        (Shape::Scissors, Shape::Rock),
        (Shape::Paper, Shape::Scissors),
        (Shape::Rock, Shape::Paper),
    ]);
    
    // mapping of opponent choice -> losing choice
    let losing_pairs = HashMap::from([
        (Shape::Rock, Shape::Scissors),
        (Shape::Scissors, Shape::Paper),
        (Shape::Paper, Shape::Rock),
    ]);

    let outcome_scores = HashMap::from([
        (RoundOutcome::Win, 6),
        (RoundOutcome::Draw, 3),
        (RoundOutcome::Lose, 0),
    ]);
    
    let shape_scores = HashMap::from([
        (Shape::Rock, 1),
        (Shape::Paper, 2),
        (Shape::Scissors, 3),
    ]);

    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Could not open file");
    let lines = contents.split("\n").collect();

    let p1_pairings = get_pairings_p1(&lines);
    let p1_score = get_score(p1_pairings, &winning_pairs, &outcome_scores, &shape_scores);

    let p2_pairings = get_pairings_p2(&lines, &winning_pairs, &losing_pairs);
    let p2_score = get_score(p2_pairings, &winning_pairs, &outcome_scores, &shape_scores);

    println!("Score with part 1 strategy: {p1_score}");
    println!("Score with part 2 strategy: {p2_score}");
}
