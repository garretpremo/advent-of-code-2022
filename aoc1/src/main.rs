use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("inputs/input1.txt").unwrap();

    let elves = parse_input(input.as_str());

    println!("1.1 answer: {}", get_max_calories(&elves));
    println!("1.2 answer: {}", get_top_three_highest_calorie_count_sum(&elves));
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.trim()
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|food| i32::from_str(food).unwrap())
                .collect()
        })
        .collect()
}

fn get_sum_of_all_elements(elf: &Vec<i32>) -> i32 {
    elf.iter().fold(0, |total_calorie_count, calorie_count| total_calorie_count + calorie_count)
}

fn get_max_calories(elves: &Vec<Vec<i32>>) -> i32 {
    let mut largest_calorie_count = 0;

    elves.iter()
        .for_each(|elf| {
            let calorie_count = get_sum_of_all_elements(elf);

            if calorie_count > largest_calorie_count {
                largest_calorie_count = calorie_count;
            }
        });

    largest_calorie_count
}

fn get_top_three_highest_calorie_count_sum(elves: &Vec<Vec<i32>>) -> i32 {
    let mut calorie_counts: Vec<i32> = elves.iter()
        .map(|elf| get_sum_of_all_elements(elf))
        .collect();

    calorie_counts.sort();
    calorie_counts.reverse();

    get_sum_of_all_elements(&calorie_counts[..3].to_vec())
}

#[test]
fn sample_part_1() {
    let sample_input = std::fs::read_to_string("../inputs/sample1.txt").unwrap();

    let elves = parse_input(sample_input.as_str());

    let largest_calorie_count = get_max_calories(&elves);

    assert_eq!(largest_calorie_count, 24000);
}

#[test]
fn sample_part_2() {
    let sample_input = std::fs::read_to_string("../inputs/sample1.txt").unwrap();

    let elves = parse_input(sample_input.as_str());

    let top_three_largest_calorie_counts= get_top_three_highest_calorie_count_sum(&elves);

    assert_eq!(top_three_largest_calorie_counts, 45000);
}
