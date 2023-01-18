use std::vec;

use crate::game::{Hand, Move};
use crate::game::Move::{PAPER, ROCK, SCISSORS};
use crate::game::Result::{DRAW, LOSS, WIN};

mod game;

fn main() {
    let strategy_guide = parse_input_file("inputs/input2.txt");

    println!("1.1 answer: {}", calculate_score_with_strategy_guide(&strategy_guide));
}

fn calculate_score_with_strategy_guide(strategy_guide: &Vec<Hand>) -> u32 {
    strategy_guide
        .iter()
        .fold(0, |acc, hand| acc + hand.score())
}

#[test]
fn sample_part_1() {
    let strategy_guide = parse_input_file("../inputs/sample2.txt");

    let score = calculate_score_with_strategy_guide(&strategy_guide);

    assert_eq!(score, 15);
}

#[test]
fn sample_part_2() {
    let strategy_guide = parse_input_file("../inputs/sample2.txt");

    let score = calculate_score_with_strategy_guide(&strategy_guide);

    assert_eq!(score, 15);
}

fn parse_input_file(path: &str) -> Vec<Hand> {
    let strategy_guide = std::fs::read_to_string(path).unwrap();

    strategy_guide.trim()
        .lines()
        .map(|hand: &str| Hand::from_chars(hand.chars().nth(0).unwrap(), hand.chars().nth(2).unwrap()))
        .collect()
}
