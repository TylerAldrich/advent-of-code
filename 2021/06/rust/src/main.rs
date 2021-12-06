use advent_input_parser::parse_by_line;

use std::collections::HashMap;
use std::io;

// NOTE: BreedingPool was my naive implementation that got through part 1.
// BreedingPoolV2 is the vastly more memory efficient version that
// can actually handle the exponential growth in part 2.

// this is an MTG reference pls don't think differently
struct BreedingPool {
    fishies: Vec<Fish>,
}

impl BreedingPool {
    // Convert a vector of sizes into lil fishies.
    fn new(timers: &Vec<u8>) -> BreedingPool {
        let mut fishies: Vec<Fish> = Vec::new();

        for timer in timers {
            fishies.push(Fish::new(*timer));
        }

        BreedingPool { fishies }
    }

    // Progress the pool by X days
    fn tick_days(&mut self, days: usize) {
        for _ in 0..days {
            let mut new_fish = 0;
            for fish in self.fishies.iter_mut() {
                if fish.tick() {
                    new_fish += 1;
                }
            }

            for _ in 0..new_fish {
                self.fishies.push(Fish::new_baby());
            }
        }
    }
}
struct Fish {
    timer: u8,
}

static DEFAULT_TIMER: u8 = 6;
static DEFAULT_BABY_TIMER: u8 = DEFAULT_TIMER + 2;

impl Fish {
    // Initializes a fish with a specific timer
    fn new(timer: u8) -> Fish {
        Fish { timer }
    }

    // Creates a new baby fish using the static DEFAULT_BABY_TIMER
    fn new_baby() -> Fish {
        Fish {
            timer: DEFAULT_BABY_TIMER,
        }
    }

    // Progresses the timer, and returns True if the timer reset
    // and a new fishie should be created
    fn tick(&mut self) -> bool {
        if self.timer == 0 {
            self.timer = DEFAULT_TIMER;
            true
        } else {
            self.timer -= 1;
            false
        }
    }
}

#[derive(Debug)]
struct BreedingPoolV2 {
    fishies: HashMap<u8, usize>,
}

impl BreedingPoolV2 {
    fn new(timers: &Vec<u8>) -> BreedingPoolV2 {
        let mut fishies = HashMap::new();

        // Fill in all keys with 0s from 0-DEFAULT_BABY_TIMER (inclusive, hence + 1)
        for k in 0..DEFAULT_BABY_TIMER + 1 {
            fishies.insert(k, 0);
        }

        for timer in timers {
            let value = fishies
                .get_mut(&timer)
                .expect(format!("No value found for key: {:?}", timer).as_str());
            *value += 1;
        }

        BreedingPoolV2 { fishies }
    }

    // Progress the pool by X days
    fn tick_days(&mut self, days: usize) {
        for _ in 0..days {
            // Amount of new fish is the number of fish @ 0
            // This is also the amount to add to the DEFAULT_TIMER
            // bucket, because they are grown up fishies that need their timer
            // reset.
            let new_fish = self.fishies.remove(&0).unwrap();

            // Handle all buckets but 0. 0 is a special case handled at the end,
            // all other buckets get their number moved down 1.
            for bucket in 1..DEFAULT_BABY_TIMER + 1 {
                let new_bucket_amount = self.fishies.remove(&bucket).unwrap();
                self.fishies.insert(bucket - 1, new_bucket_amount);
            }

            // This key is now empty, now that we've moved down all fish.
            self.fishies.insert(DEFAULT_BABY_TIMER, new_fish);
            // The key at DEFAULT_TIMER is not empty yet, so we add to it
            self.fishies.insert(
                DEFAULT_TIMER,
                self.fishies.get(&DEFAULT_TIMER).unwrap() + new_fish,
            );
        }
    }

    fn total_fish(&self) -> usize {
        self.fishies.values().sum()
    }
}

fn main() {
    let parsed_input = parse_by_line(io::stdin().lock());
    // This problem only has 1 line of input.
    let parsed_input = parsed_input
        .get(0)
        .expect("One line of input was expected!");

    let mut timers: Vec<u8> = Vec::new();
    for timer in parsed_input.split(",") {
        timers.push(
            timer
                .parse::<u8>()
                .expect("Expected an integer no larger than a u8"),
        );
    }

    let mut pool = BreedingPool::new(&timers);
    pool.tick_days(80);
    println!(
        "Fish in the pool after 80 days (part 1): {:?}",
        pool.fishies.len()
    );

    // LOL for thinking this naive implementation would work.
    // Thanks to exponential growth, even if each fish is 1 byte,
    // after 256 days there will be 26984457539 fish (with sample input).
    // This is ~27gb stored in a vec, which isn't gunna work too well.

    // pool.tick_days(256 - 80);
    // println!(
    //     "Number of fish in the pool after 256 days (part 1): {:?}",
    //     pool.fishies.len()
    // );

    let mut better_pool = BreedingPoolV2::new(&timers);
    better_pool.tick_days(256);
    println!(
        "Fish in the pool after 256 days (part 1): {:?}",
        better_pool.total_fish()
    );
}
