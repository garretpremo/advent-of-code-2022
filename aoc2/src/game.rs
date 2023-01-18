use crate::game::Move::{PAPER, ROCK, SCISSORS};
use crate::game::Result::{DRAW, LOSS, WIN};

pub enum Move { ROCK, PAPER, SCISSORS }
#[derive(Debug)]
pub enum Result { WIN, LOSS, DRAW }

impl Move {
    pub fn from(input: char) -> Move {
        match input {
            'A' | 'X' => ROCK,
            'B' | 'Y' => PAPER,
            'C' | 'Z' => SCISSORS,
            _ => panic!("invalid RPS input")
        }
    }

    pub fn score(&self) -> u32 {
        match self { ROCK => 1, PAPER => 2, SCISSORS => 3 }
    }
}

impl Result {
    pub fn score(&self) -> u32 {
        match self { WIN => 6, DRAW => 3, LOSS => 0 }
    }
}

impl PartialEq for Result {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (WIN, WIN) => true,
            (DRAW, DRAW) => true,
            (LOSS, LOSS) => true,
            _ => false
        }
    }
}

pub struct Hand {
    pub your_move: Move,
    pub my_move: Move
}

impl Hand {
    pub fn from_chars(your_move: char, my_move: char) -> Hand {
        Hand {
            your_move: Move::from(your_move),
            my_move: Move::from(my_move)
        }
    }

    pub fn from_moves(your_move: Move, my_move: Move) -> Hand {
        Hand { your_move, my_move }
    }

    pub fn score(&self) -> u32 {
        self.result().score() + self.my_move.score()
    }

    pub fn result(&self) -> Result {
        match &self.your_move {
            ROCK     => match &self.my_move { ROCK => DRAW, PAPER => WIN, SCISSORS => LOSS },
            PAPER    => match &self.my_move { ROCK => LOSS, PAPER => DRAW, SCISSORS => WIN },
            SCISSORS => match &self.my_move { ROCK => WIN, PAPER => LOSS, SCISSORS => DRAW },
        }
    }
}

#[test]
fn test_hand_result() {
    {
        let draw_hands = vec![
            Hand::from_moves(ROCK, ROCK),
            Hand::from_moves(PAPER, PAPER),
            Hand::from_moves(SCISSORS, SCISSORS)
        ];

        draw_hands.iter().for_each(|hand| { assert_eq!(hand.result(), DRAW) });
    }

    {
        let win_hands = vec![
            Hand::from_moves(SCISSORS, ROCK),
            Hand::from_moves(ROCK, PAPER),
            Hand::from_moves(PAPER, SCISSORS)
        ];

        win_hands.iter().for_each(|hand| { assert_eq!(hand.result(), WIN) });
    }

    {
        let loss_hands = vec![
            Hand::from_moves(ROCK, SCISSORS),
            Hand::from_moves(PAPER, ROCK),
            Hand::from_moves(SCISSORS, PAPER)
        ];

        loss_hands.iter().for_each(|hand| { assert_eq!(hand.result(), LOSS) });
    }
}
