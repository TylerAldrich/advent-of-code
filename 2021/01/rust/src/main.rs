use advent_input_parser::parse_by_line;
use std::collections::VecDeque;
use std::io;

fn main() {
    let parsed_input = parse_by_line(io::stdin().lock());

    let mut depths: VecDeque<usize> = VecDeque::new();
    for value in parsed_input {
        depths.push_back(
            value
                .parse::<usize>()
                .expect("Expected an unsigned integer!"),
        );
    }

    let part_1_answer = count_depth_increases(depths.clone());
    println!("Part 1 answer: {:?}", part_1_answer);

    let part_2_answer = count_sliding_window_depth_increases(depths.clone());
    println!("Part 2 answer: {:?}", part_2_answer);
}

// Count the number of times the depth increases.
// E.g. [102, 100, 105] => 1 increase
//      [100, 101, 102, 103] => 3 increases
fn count_depth_increases(mut depths: VecDeque<usize>) -> usize {
    let mut last_depth = depths.pop_front().unwrap();
    let mut number_of_increases = 0;

    for depth in depths {
        if depth > last_depth {
            number_of_increases += 1;
        }
        last_depth = depth;
    }

    number_of_increases
}

// Count the number of times the depth increases in a sliding window of size 3
// Groups of 3 depths are added together and the sliding windows are then compared in order.
// E.g. [100, 101, 102, 103, 104, 105] => 4 increases in windows [100, 101, 102], [101, 102, 103], etc.
fn count_sliding_window_depth_increases(depths: VecDeque<usize>) -> usize {
    // Queue that will contain at most 3 values, and will track the current depth windows.
    let mut depth_windows: VecDeque<usize> = VecDeque::new();

    let mut number_of_increases = 0;
    let mut last_window: Option<usize> = None;

    for depth in depths {
        // Add depth to all currently existing windows
        for window in depth_windows.iter_mut() {
            *window += depth;
        }

        // Add the current depth to the end of our queue
        depth_windows.push_back(depth);

        // If the queue is now size = 3 [window size], pop the first value,
        // which is our "last window" to compare against.
        if depth_windows.len() == 3 {
            let new_last_window = depth_windows.pop_front().unwrap();

            // If we have a last window to compare against, do so.
            if let Some(value) = last_window {
                if new_last_window > value {
                    number_of_increases += 1;
                }
            }

            last_window = Some(new_last_window);
        }
    }

    number_of_increases
}
