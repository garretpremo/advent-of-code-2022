use std::collections::VecDeque;
use std::usize;

#[derive(Debug, Clone)]
pub struct Cargo {
    stacks: Vec<VecDeque<char>>,
    maneuvers: Vec<Maneuver>
}

#[derive(Debug, Clone)]
struct Maneuver {
    moving: usize,
    from: usize,
    to: usize
}

impl Cargo {
    // public function is immutable, so that the original data is not mutated
    pub fn move_crates(&self, keep_order: bool) -> Cargo {
        let mut cargo = self.clone();

        cargo.do_move_crates(keep_order);

        cargo
    }

    fn do_move_crates(&mut self, keep_order: bool) {
        self.maneuvers.iter()
            .for_each(|maneuver| {
                let mut crates = vec![];

                for _ in 0..maneuver.moving {
                    crates.push(self.stacks[maneuver.from - 1].pop_front().unwrap());
                }

                if keep_order {
                    crates.reverse();
                }

                crates.into_iter().for_each(|_crate| {
                    self.stacks[maneuver.to - 1].push_front(_crate);
                });
            });

        self.maneuvers = vec![];
    }

    pub fn get_top_crates(&self) -> String {
        self.stacks.iter()
            .map(|stack| stack.get(0).unwrap())
            .collect()
    }

    pub fn parse(input: String) -> Cargo {
        let mut stacks = vec![];
        let mut moves = vec![];
        let mut parsing_moves = false;

        input.lines()
            .for_each(|line| {
                if line == "" {
                    parsing_moves = true;
                    return;
                }

                if parsing_moves {
                    moves.push(Maneuver::parse(line));
                    return;
                } else if !line.contains("[") {
                    return;
                }

                let mut chars= line.chars();

                for i in 0..(line.len() + 4) / 4 {
                    chars.next();
                    match chars.next() {
                        Some(' ') | None => {},
                        Some(crate_name) => {
                            while i >= stacks.len() {
                                stacks.push(VecDeque::new());
                            }

                            stacks[i].push_back(crate_name);
                        }
                    }
                    chars.next();
                    chars.next();
                }
            });

        Cargo { stacks, maneuvers: moves }
    }
}

impl Maneuver {
    pub fn parse(input: &str) -> Maneuver {
        let mut split = input.rsplit(" ");

        let to = usize::from_str_radix(split.next().unwrap(), 10).unwrap();

        assert_eq!(split.next().unwrap(), "to");

        let from = usize::from_str_radix(split.next().unwrap(), 10).unwrap();

        assert_eq!(split.next().unwrap(), "from");

        let moving = usize::from_str_radix(split.next().unwrap(), 10).unwrap();

        Maneuver { moving, from, to }
    }
}

#[test]
fn test_move_parse() {
    let maneuver = Maneuver::parse("move 1 from 2 to 1");

    assert_eq!(maneuver.moving, 1);
    assert_eq!(maneuver.from, 2);
    assert_eq!(maneuver.to, 1);
}