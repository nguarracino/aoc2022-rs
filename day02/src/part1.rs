#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn normalize(s: &str) -> Move {
    match s {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        _ => panic!("Unknown move: {:?}", s),
    }
}

fn move_score(s: &str) -> u32 {
    match normalize(&s) {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn outcome_score(s1: &str, s2: &str) -> u32 {
    let first = normalize(&s1);
    let second = normalize(&s2);

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

fn main() {
    let moves: Vec<Vec<String>> = include_str!("../input.txt")
        .lines()
        .map(|line| Vec::from_iter(line.split(' ').map(String::from)))
        .collect();

    let mut total = 0;
    for line in moves.iter() {
        total += outcome_score(&line[0], &line[1]) + move_score(&line[1]);
    }

    println!("{:?}", total);
}
