use crate::cargo::Cargo;

mod cargo;

fn main() {
    let cargo = parse_input_file("inputs/input5.txt");

    println!("5.1 answer: {}", cargo.move_crates(false).get_top_crates());
    println!("5.2 answer: {}", cargo.move_crates(true).get_top_crates());
}

#[test]
fn sample_part_1() {
    let cargo = parse_input_file("../inputs/sample5.txt");

    let cargo = cargo.move_crates(false);

    assert_eq!(cargo.get_top_crates(), String::from("CMZ"));
}

#[test]
fn sample_part_2() {
    let cargo = parse_input_file("../inputs/sample5.txt");

    let cargo = cargo.move_crates(true);

    assert_eq!(cargo.get_top_crates(), String::from("MCD"));
}

fn parse_input_file(path: &str) -> Cargo {
    let input = std::fs::read_to_string(path).unwrap();

    Cargo::parse(input)
}
