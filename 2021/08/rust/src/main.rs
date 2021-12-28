use advent_input_parser::parse_by_line;
use std::{
    collections::{HashMap, HashSet},
    io,
};

fn main() {
    let parsed_input = parse_by_line(io::stdin().lock());

    let unique_segment_digits = part_1(&parsed_input);
    println!(
        "(Part 1) Amount of digits 1, 4, 7, 8 appearing: {:?}",
        unique_segment_digits
    );

    let total_values = part_2(&parsed_input);
    println!("(Part 2) Total of all outputs: {:?}", total_values);
}

// Counts the amount of strings that are of length 2, 3, 4, or 7
// in the input.
fn part_1(input_lines: &Vec<String>) -> usize {
    let mut unique_segment_digits = 0;

    for line in input_lines {
        let split_str = line.split("|");
        let second_half = split_str.last().unwrap();

        for digits in second_half.split(" ") {
            if digits.len() == 2 || digits.len() == 3 || digits.len() == 4 || digits.len() == 7 {
                unique_segment_digits += 1;
            }
        }
    }

    unique_segment_digits
}

fn part_2(input_lines: &Vec<String>) -> usize {
    let mut total_value = 0;

    // These confusingly written rules are the way to decode any given number
    // Using numbers 1, 4, 7, and 8, these rules + the length of the string
    // can be used to determine what each other number is.

    // 0: Shares all letters with 1 + 7, 3/4 of 4's letters, 6 letters total
    // 1: (Unique) 2 letters
    // 2: Shares 1 letter with 1, 2 letters with 7, 2 letters with 4, 5 letters total
    // 3: Shares all letters with 1 + 7, 3/4 of 4's letters, 5 letters total
    // 4: (Unique) 4 letters - Shares 2 letters with 1
    // 5: Shares 1 letter with 1, 2 letters with 7, 3 letters with 4, 5 letters total
    // 6: Shares 1 letter with 1, 2 letters with 7, 3 letters with 4, 6 letters total
    // 7: (Unique) 3 letters - Shares 2 letters with 1
    // 8: (Unique) 7 letters
    // 9: Shares all letters with 1 + 4 + 7, 6 letters total

    for line in input_lines {
        let mut split_str = line.split("|");
        // Incredibly ugly, but this turns a Vec like ["eb", "bcdfgea"] into ["be", "abcdefg"]
        // so that the order of letters is consistent between input/output.
        let first_half: Vec<String> = split_str
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|e| {
                let mut letters: Vec<char> = e.chars().collect();
                letters.sort_unstable();
                letters.into_iter().collect()
            })
            .collect();
        let second_half: Vec<String> = split_str
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|e| {
                let mut letters: Vec<char> = e.chars().collect();
                letters.sort_unstable();
                letters.into_iter().collect()
            })
            .collect();

        // println!("{:?}, {:?}", first_half, second_half);

        let mut digit_map: HashMap<usize, &String> = HashMap::new();
        let mut value_map: HashMap<&String, usize> = HashMap::new();

        // First pass: Get 1, 4, 7 and 8 into our hashmap ahead of pass 2.
        for value in first_half.iter() {
            match value.len() {
                2 => {
                    digit_map.insert(1, value);
                    value_map.insert(value, 1);
                }
                3 => {
                    digit_map.insert(7, value);
                    value_map.insert(value, 7);
                }
                4 => {
                    digit_map.insert(4, value);
                    value_map.insert(value, 4);
                }
                7 => {
                    digit_map.insert(8, value);
                    value_map.insert(value, 8);
                }
                _ => (),
            }
        }

        for value in first_half.iter() {
            match value.len() {
                5 => {
                    // 2: Shares 1 letter with 1, 2 letters with 7, 2 letters with 4, 5 letters total
                    // 3: Shares all letters with 1 + 7, 3 of 4's letters, 5 letters total
                    // 5: Shares 1 letter with 1, 2 letters with 7, 3 letters with 4, 5 letters total

                    // Note: The only difference between 2 and 3+5 is the amount of letters they share with 4.
                    let four_value = digit_map.get(&4).unwrap();
                    let amount_shared = shared_chars(value, four_value);
                    if amount_shared == 2 {
                        digit_map.insert(2, value);
                        value_map.insert(value, 2);
                    } else if amount_shared == 3 {
                        let seven_value = digit_map.get(&7).unwrap();
                        let amount_shared_seven = shared_chars(value, seven_value);
                        // The difference between 3 and 5 is the number of digits shared with 7
                        if amount_shared_seven == seven_value.len() {
                            digit_map.insert(3, value);
                            value_map.insert(value, 3);
                        } else {
                            digit_map.insert(5, value);
                            value_map.insert(value, 5);
                        }
                    } else {
                        panic!("Unexpected amount_shared in 5 branch: {:?}", amount_shared);
                    }
                }
                6 => {
                    // 0: Shares all letters with 1 + 7, 3 of 4's letters, 6 letters total
                    // 6: Shares 1 letter with 1, 2 letters with 7, 3 letters with 4, 6 letters total
                    // 9: Shares all letters with 1 + 4 + 7, 6 letters total

                    // If all letters are shared with 4, it must be a 9
                    let four_value = digit_map.get(&4).unwrap();
                    let amount_shared = shared_chars(value, four_value);
                    if amount_shared == four_value.len() {
                        digit_map.insert(9, value);
                        value_map.insert(value, 9);
                    } else {
                        // Now, if all of 7's letters are shared, it must be a 0
                        let seven_value = digit_map.get(&7).unwrap();
                        let amount_shared = shared_chars(value, seven_value);
                        if amount_shared == seven_value.len() {
                            digit_map.insert(0, value);
                            value_map.insert(value, 0);
                        } else {
                            // If not all of 7s letters are shared, it must be a 6
                            digit_map.insert(6, value);
                            value_map.insert(value, 6);
                        }
                    }
                }
                _ => (),
            }
        }

        // Now, we can look at second_half and figure out what each number is.
        // second_half is always 4 digits, so to get total_value, the first number is multiplied by 1000, then 100, then 10, then 1
        let mut multiplier: usize = 1000;
        for value in second_half {
            let number = value_map.get(&value).unwrap();
            // println!("Value for {:?}: {:?}", value, number);
            total_value += number * multiplier;
            multiplier /= 10;
        }
    }

    total_value
}

fn shared_chars(str_1: &String, str_2: &String) -> usize {
    let chars_1: HashSet<char> = str_1.chars().collect();
    let chars_2: HashSet<char> = str_2.chars().collect();

    chars_1.intersection(&chars_2).count()
}
