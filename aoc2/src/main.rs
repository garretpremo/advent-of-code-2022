use crate::game::{Code, Hand};

mod game;

fn main() {
    let strategy_guide = parse_input_file("inputs/input2.txt");

    println!("1.1 answer: {}", calculate_score_with_strategy_guide(&strategy_guide, false));
    println!("1.2 answer: {}", calculate_score_with_strategy_guide(&strategy_guide, true));
}

fn calculate_score_with_strategy_guide(strategy_guide: &Vec<(Code, Code)>, part_2: bool) -> u32 {
    strategy_guide.iter()
        .map(|(code_a, code_b)| {
            Hand::from_codes(code_a, code_b, part_2)
        })
        .collect::<Vec<Hand>>()
        .iter()
        .fold(0, |acc, hand| acc + hand.score())
}

#[test]
fn sample_part_1() {
    let strategy_guide = parse_input_file("../inputs/sample2.txt");

    let score = calculate_score_with_strategy_guide(&strategy_guide, false);

    assert_eq!(score, 15);
}

#[test]
fn sample_part_2() {
    let strategy_guide = parse_input_file("../inputs/sample2.txt");

    let score = calculate_score_with_strategy_guide(&strategy_guide, true);

    assert_eq!(score, 12);
}

fn parse_input_file(path: &str) -> Vec<(Code, Code)> {
    let strategy_guide = std::fs::read_to_string(path).unwrap();

    strategy_guide.trim()
        .lines()
        .map(|hand: &str| {
            let first_code = Code::from(hand.chars().nth(0).unwrap());
            let second_code = Code::from(hand.chars().nth(2).unwrap());
            (first_code, second_code)
        })
        .collect()
}
