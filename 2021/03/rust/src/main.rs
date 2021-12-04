use advent_input_parser::parse_by_line;
use std::{convert::TryInto, io};

struct GammaEpsilon {
    pub gamma_rate: usize,
    pub epsilon_rate: usize,
}

struct OxyCORating {
    pub oxygen_rating: usize,
    pub co2_rating: usize,
}

fn main() {
    let parsed_input = parse_by_line(io::stdin().lock());

    let mut numbers: Vec<usize> = Vec::new();
    // Input is all the same length in binary.
    let mut binary_length = 0;
    for value in parsed_input {
        if binary_length == 0 {
            binary_length = value.len();
        }
        let integer_value = usize::from_str_radix(&value, 2).expect("Expected a binary number!");
        numbers.push(integer_value);
    }

    part_1(&numbers, binary_length);
    part_2(&numbers, binary_length);
}

fn part_1(numbers: &Vec<usize>, binary_length: usize) {
    let ge_rates = gamma_epsilon_rate(numbers, binary_length);

    println!(
        "Gamma: {:12b}, Epsilon: {:12b}, Power Consumption: {:?}",
        ge_rates.gamma_rate,
        ge_rates.epsilon_rate,
        ge_rates.gamma_rate * ge_rates.epsilon_rate
    );
    println!(
        "Gamma: {:?}, Epsilon: {:?}, Power Consumption: {:?}",
        ge_rates.gamma_rate,
        ge_rates.epsilon_rate,
        ge_rates.gamma_rate * ge_rates.epsilon_rate
    );
}

fn part_2(numbers: &Vec<usize>, binary_length: usize) {
    let ratings = oxygen_and_co2_rating(numbers, binary_length);

    println!(
        "Oxygen Rating: {:12b}, {:?}",
        ratings.oxygen_rating, ratings.oxygen_rating
    );
    println!(
        "CO2 Rating: {:12b}, {:?}",
        ratings.co2_rating, ratings.co2_rating
    );
    println!(
        "Final value: {:?}",
        ratings.oxygen_rating * ratings.co2_rating
    )
}

fn oxygen_and_co2_rating(numbers: &Vec<usize>, binary_length: usize) -> OxyCORating {
    let mut valid_oxygen_numbers = numbers.clone();
    let mut valid_co2_numbers = numbers.clone();
    let exp_base: usize = 2;

    // Start by looking at the leftmost bit.
    let mut offset = binary_length;

    while valid_oxygen_numbers.len() > 1 || valid_co2_numbers.len() > 1 {
        offset -= 1;

        let mask = exp_base.pow(offset.try_into().unwrap());

        if valid_oxygen_numbers.len() > 1 {
            let gamma_rate = gamma_epsilon_rate(&valid_oxygen_numbers, binary_length).gamma_rate;
            let gamma_rate_at_offset = (gamma_rate & mask) >> offset;

            valid_oxygen_numbers = valid_oxygen_numbers
                .into_iter()
                .filter(|number| (number & mask) >> offset == gamma_rate_at_offset)
                .collect();
        }

        if valid_co2_numbers.len() > 1 {
            let epsilon_rate = gamma_epsilon_rate(&valid_co2_numbers, binary_length).epsilon_rate;
            let epsilon_rate_at_offset = (epsilon_rate & mask) >> offset;

            valid_co2_numbers = valid_co2_numbers
                .into_iter()
                .filter(|number| (number & mask) >> offset == epsilon_rate_at_offset)
                .collect();
        }
    }

    OxyCORating {
        oxygen_rating: valid_oxygen_numbers.pop().unwrap(),
        co2_rating: valid_co2_numbers.pop().unwrap(),
    }
}

fn gamma_epsilon_rate(numbers: &Vec<usize>, binary_length: usize) -> GammaEpsilon {
    let exp_base: usize = 2;

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for x in 0..binary_length {
        let offset = binary_length - 1 - x;
        let mask = exp_base.pow(offset.try_into().unwrap());

        let mut leaning_gamma = 0;
        let mut leaning_epsilon = 0;
        for number in numbers {
            let bit = (number & mask) >> offset;

            if bit == 1 {
                leaning_gamma += 1
            } else {
                leaning_epsilon += 1
            }
        }

        if leaning_gamma >= leaning_epsilon {
            gamma_rate ^= mask;
        } else {
            epsilon_rate ^= mask;
        }
    }

    GammaEpsilon {
        gamma_rate,
        epsilon_rate,
    }
}
