use std::collections::HashSet;

fn main() {
    let input = parse_input_file("inputs/input6.txt");

    println!("6.1 answer: {}", find_first_marker(&input, 4));
}

#[test]
fn test_sample_part_1() {
    let sample = parse_input_file("../inputs/sample6.txt");

    assert_eq!(find_first_marker(&sample, 4), 7);
}

fn parse_input_file(path: &str) -> String {
    String::from(std::fs::read_to_string(path).unwrap().trim())
}

fn find_first_marker(input: &String, distinct_character_count: usize) -> usize {
    let mut characters: Vec<char> = vec![];

    for (index, character) in input.chars().enumerate() {
        characters.push(character);

        if index < (distinct_character_count - 1) {
            continue;
        }

        let character_set: HashSet<char> = characters.iter().copied().collect();

        if character_set.len() == characters.len() {
            return index + 1
        }

        characters.remove(0);
    }

    0
}
