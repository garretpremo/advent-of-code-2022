use crate::game::Code::{A, B, C, X, Y, Z};
use crate::game::Move::{PAPER, ROCK, SCISSORS};
use crate::game::Result::{DRAW, LOSS, WIN};

pub enum Code { A, B, C, X, Y, Z }
pub enum Move { ROCK, PAPER, SCISSORS }
#[derive(Debug)]
pub enum Result { WIN, LOSS, DRAW }

impl Code {
    pub fn from(input: char) -> Code {
        match input {
            'A' => A,
            'B' => B,
            'C' => C,
            'X' => X,
            'Y' => Y,
            'Z' => Z,
            _ => panic!("invalid code")
        }
    }

    pub fn to_move(&self) -> Move {
        match self {
            A | X => ROCK,
            B | Y => PAPER,
            C | Z => SCISSORS,
        }
    }
}

impl Move {
    pub fn get_losing_move(other: &Move) -> Move {
        match other { ROCK => SCISSORS, PAPER => ROCK, SCISSORS => PAPER }
    }

    pub fn get_winning_move(other: &Move) -> Move {
        match other { ROCK => PAPER, PAPER => SCISSORS, SCISSORS => ROCK }
    }

    pub fn get_draw(other: &Move) -> Move {
        match other { ROCK => ROCK, PAPER => PAPER, SCISSORS => SCISSORS }
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
            (WIN, WIN) | (DRAW, DRAW) | (LOSS, LOSS) => true,
            _ => false
        }
    }
}

pub struct Hand {
    pub your_move: Move,
    pub my_move: Move
}

#[allow(dead_code)]
impl Hand {
    pub fn from_moves(your_move: Move, my_move: Move) -> Hand {
        Hand { your_move, my_move }
    }

    pub fn from_codes(first_code: &Code, second_code: &Code, part_2: bool) -> Hand {
        if !part_2 {
            return Hand::from_moves(first_code.to_move(), second_code.to_move())
        }

        let your_move = first_code.to_move();

        let my_move = match second_code {
            X => Move::get_losing_move(&your_move),
            Y => Move::get_draw(&your_move),
            Z => Move::get_winning_move(&your_move),
            _ => panic!("invalid second code")
        };

        Hand::from_moves(your_move, my_move)
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
