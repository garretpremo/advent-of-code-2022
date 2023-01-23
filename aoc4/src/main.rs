use pair::AssignmentPair;

mod pair;

fn main() {
    let assignment_pairs = parse_input_file("inputs/input4.txt");

    let enveloping_pairs = get_enveloping_pair_count(&assignment_pairs);
    let overlapping_pairs = get_overlapping_pair_count(&assignment_pairs);

    println!("4.1 answer: {}", enveloping_pairs);
    println!("4.2 answer: {}", overlapping_pairs);
}

#[test]
fn sample_part_1() {
    let assignment_pairs = parse_input_file("../inputs/sample4.txt");

    assert_eq!(assignment_pairs.len(), 6);

    let enveloping_pairs = get_enveloping_pair_count(&assignment_pairs);

    assert_eq!(enveloping_pairs, 2);
}

#[test]
fn sample_part_2() {
    let assignment_pairs = parse_input_file("../inputs/sample4.txt");

    let overlapping_pairs = get_overlapping_pair_count(&assignment_pairs);

    assert_eq!(overlapping_pairs, 4);
}

fn get_enveloping_pair_count(assignment_pairs: &Vec<AssignmentPair>) -> u32 {
    assignment_pairs.iter()
        .fold(0, |count, pair| {
            match pair.one_assignment_envelops_another() {
                true => count + 1,
                false => count
            }
        })
}

fn get_overlapping_pair_count(assignment_pairs: &Vec<AssignmentPair>) -> u32 {
    assignment_pairs.iter()
        .fold(0, |count, pair| {
            match pair.has_overlapping_assignments() {
                true => count + 1,
                false => count
            }
        })
}

fn parse_input_file(path: &str) -> Vec<AssignmentPair> {
    let input = std::fs::read_to_string(path).unwrap();

    input.trim()
    .lines()
    .map(AssignmentPair::from)
    .collect()
}