use crate::rucksack::Rucksack;

mod rucksack;

fn main() {
    let rucksacks = parse_input_file("inputs/input3.txt");

    println!("3.1 answer: {}", get_rearrangement_priorities_summed(&rucksacks));
}

#[test]
fn sample_part_1() {
    let rucksacks = parse_input_file("../inputs/sample3.txt");

    assert_eq!(get_rearrangement_priorities_summed(&rucksacks), 157);
}

fn get_rearrangement_priorities_summed(rucksacks: &Vec<Rucksack>) -> u32 {
    rucksacks.iter()
        .fold(0, |priority_sum, rucksack| {
            priority_sum + get_item_priority(rucksack.get_common_item())
        })
}

fn get_item_priority(item: char) -> u32 {
    let ascii_value = item as u32;

    match ascii_value {
        97..=123 => ascii_value - 96, // a-z maps to 1-26
        65..=96 => ascii_value - 38,  // A-Z maps to 27-52
        _ => panic!("invalid item")
    }
}

#[test]
fn test_get_item_priority() {
    assert_eq!(get_item_priority('a'), 1);
    assert_eq!(get_item_priority('z'), 26);
    assert_eq!(get_item_priority('A'), 27);
    assert_eq!(get_item_priority('Z'), 52);
}

fn parse_input_file(path: &str) -> Vec<Rucksack> {
    let input = std::fs::read_to_string(path).unwrap();

    input.trim().lines()
        .map(|line| {
            let item_count = line.len();
            let compartment_one = &line[..item_count/2];
            let compartment_two = &line[item_count/2..];

            Rucksack::from(compartment_one, compartment_two)
        })
        .collect()
}
