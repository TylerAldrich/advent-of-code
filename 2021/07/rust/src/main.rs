use advent_input_parser::parse_by_line;
use std::{convert::TryInto, io};

fn main() {
    let parsed_input = parse_by_line(io::stdin().lock());
    // This problem only has 1 line of input.
    let parsed_input = parsed_input
        .get(0)
        .expect("One line of input was expected!");

    let mut max_position: u32 = 0;
    let mut initial_positions: Vec<u32> = Vec::new();

    for pos in parsed_input.split(",") {
        let pos_int = pos
            .parse::<u32>()
            .expect("Expected an integer no larger than a u32.");
        initial_positions.push(pos_int);
        max_position = max_position.max(pos_int);
    }

    println!("Max position: {:?}", max_position);

    let part_1_position = part_1(&initial_positions, max_position);
    println!("Part 1 answer: {:?}", part_1_position);

    let part_2_position = part_2(&initial_positions, max_position);
    println!("Part 2 answer: {:?}", part_2_position);
}

// Returns the position that uses the least "fuel"
// i.e., the position that is the least cumulative distance from all
// values in "start_positions".
fn part_1(start_positions: &Vec<u32>, max_position: u32) -> u32 {
    let mut min_distance = u32::MAX;
    let mut min_distance_position = u32::MAX;

    for i in 0..max_position {
        let mut distance = 0;
        for position in start_positions {
            distance += (i64::from(*position) - i64::from(i)).abs()
        }

        let distance: u32 = distance.try_into().unwrap();
        if distance < min_distance {
            min_distance = distance;
            min_distance_position = i;
        }
    }

    println!("Part 1 min_distance_position: {:?}", min_distance_position);
    min_distance
}

// Returns the position that uses the most "fuel"
// i.e., the position that is the least cumulative distance from all
// values in "start_positions". The difference between this and part 1
// is that fuel cost for distance is no longer abs(a - b),
// and is now sum(1 -> abs(a-b))
fn part_2(start_positions: &Vec<u32>, max_position: u32) -> u32 {
    let mut min_distance = u32::MAX;
    let mut min_distance_position = u32::MAX;

    // Create a vector to store the fuel cost for a given distance
    // We can do this by initializing a vector from 1 -> X
    // and then going from 2 -> X and adding the previous value.
    // e.g.
    // > distance_cost = [0, 1, 2, 3, 4]
    // becomes
    // > distance_cost = [0, 1, 3, 6, 10]
    // and arr[4] gives us 10 immediately.
    let distance_cost_base: Vec<u32> = (0..max_position + 1).collect();
    let mut distance_cost: Vec<u32> = Vec::new();
    distance_cost.push(0);
    for val in distance_cost_base.into_iter().skip(1) {
        let last_val = *distance_cost.last().unwrap();
        distance_cost.push(val + last_val);
    }

    for i in 0..max_position {
        let mut distance: u32 = 0;
        for position in start_positions {
            let distance_length: usize = (i64::from(*position) - i64::from(i))
                .abs()
                .try_into()
                .unwrap();
            distance += distance_cost[distance_length];
        }

        let distance: u32 = distance.try_into().unwrap();
        if distance < min_distance {
            min_distance = distance;
            min_distance_position = i;
        }
    }

    println!("Part 2 min_distance_position: {:?}", min_distance_position);
    min_distance
}
