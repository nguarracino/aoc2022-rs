enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw
}

fn normalize_move(s: &str) -> Move {
    match s {
        "A"=> Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _ => panic!("Unknown move: {:?}", s),
    }
}

fn normalize_outcome(s: &str) -> Outcome {
    match s {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Unknown outcome: {:?}", s),
    }
}

fn move_score(s: &str) -> u32 {
    match normalize_move(&s) {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn outcome_score(s1: &str, s2: &str) -> u32 {
    let first = normalize_move(&s1);
    let second = normalize_move(&s2);

    match (first, second) {
        (Move::Rock, Move::Scissors)
        | (Move::Paper, Move::Rock)
        | (Move::Scissors, Move::Paper) => 0,
        (Move::Rock, Move::Rock)
        | (Move::Paper, Move::Paper)
        | (Move::Scissors, Move::Scissors) => 3,
        (Move::Rock, Move::Paper)
        | (Move::Paper, Move::Scissors)
        | (Move::Scissors, Move::Rock) => 6,
    }
}

fn calculate_move(s1: &str, s2: &str) -> String {
    let first = normalize_move(&s1);
    let outcome = normalize_outcome(&s2);

    match outcome {
        Outcome::Lose => match first {
            Move::Rock => "C".to_string(),
            Move::Paper => "A".to_string(),
            Move::Scissors => "B".to_string(),
        },
        Outcome::Draw => match first {
            Move::Rock => "A".to_string(),
            Move::Paper => "B".to_string(),
            Move::Scissors => "C".to_string(),
        },
        Outcome::Win => match first {
            Move::Rock => "B".to_string(),
            Move::Paper => "C".to_string(),
            Move::Scissors => "A".to_string(),
        },
    }
}

fn main() {
    let moves: Vec<Vec<String>> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let split = Vec::from_iter(line.split(' '));
            vec![String::from(split[0]), calculate_move(split[0], split[1])]
        })
        .collect();

    let mut total = 0;
    for line in moves.iter() {
        total += outcome_score(&line[0], &line[1]) + move_score(&line[1]);
    }

    println!("{:?}", total);
}
