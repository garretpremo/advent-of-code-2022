use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("inputs/input1.txt").unwrap();

    let elves = parse_input(input.as_str());

    println!("1.1 answer: {}", get_max_calories(elves));
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

fn get_max_calories(elves: Vec<Vec<i32>>) -> i32 {
    let mut largest_calorie_count = 0;

    elves.iter()
        .for_each(|elf| {
            let mut calorie_count = 0;

            elf.iter().for_each(|calories| calorie_count += calories);

            if calorie_count > largest_calorie_count {
                largest_calorie_count = calorie_count;
            }
        });

    largest_calorie_count
}

#[test]
fn sample() {
    let sample_input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";

    let elves = parse_input(sample_input);

    let largest_calorie_count = get_max_calories(elves);

    assert_eq!(largest_calorie_count, 24000);
}
