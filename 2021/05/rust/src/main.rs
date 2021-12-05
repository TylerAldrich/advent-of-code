use advent_input_parser::parse_by_line;
use std::cmp;
use std::io;

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: Option<&str>, y: Option<&str>) -> Point {
        Point {
            x: Point::from_option_str(x),
            y: Point::from_option_str(y),
        }
    }

    fn from_option_str(value: Option<&str>) -> usize {
        value
            .unwrap()
            .parse::<usize>()
            .expect("Point must be an integer!")
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start_coord: Option<&str>, end_coord: Option<&str>) -> Line {
        let mut start_split = start_coord.unwrap().split(",");
        let mut end_split = end_coord.unwrap().split(",");
        Line {
            start: Point::new(start_split.next(), start_split.next()),
            end: Point::new(end_split.next(), end_split.next()),
        }
    }

    fn start(&self) -> &Point {
        &self.start
    }

    fn end(&self) -> &Point {
        &self.end
    }
}

// This is very similar to our BingoBoard from day 4
// The main difference is (besides the inner struct being a Coordinate)
// that the x/y axis range is unknown initially.
#[derive(Debug)]
struct Map {
    coordinates: Vec<Coordinate>,
    size: usize,
}

impl Map {
    // Initialize a new empty map using the known x/y max.
    fn new(size: usize) -> Map {
        // TODO: create x*y empty Coordinate for coordinates

        let mut coordinates: Vec<Coordinate> = Vec::new();
        // Account for the 0 row - size of 9 means the board goes from
        // 0->9, which is 10 values.
        let size = size + 1;

        for _ in 0..size * size {
            coordinates.push(Coordinate::new());
        }

        println!("Created map with length: {:?}", coordinates.len());

        Map { coordinates, size }
    }

    // Given a row and col, return the offset into the vec that corresponds.
    fn vec_offset(&self, row: usize, col: usize) -> usize {
        row * self.size + col
    }

    fn mark_line(&mut self, line: &Line, consider_diagonals: bool) {
        let start_point = line.start();
        let end_point = line.end();

        if start_point.x != end_point.x && start_point.y != end_point.y {
            if consider_diagonals {
                let mut x_value = start_point.x;
                let mut y_value = start_point.y;

                while x_value != end_point.x && y_value != end_point.y {
                    let offset = self.vec_offset(x_value, y_value);
                    self.coordinates.get_mut(offset).unwrap().mark();

                    if x_value < end_point.x {
                        x_value += 1;
                    } else {
                        x_value -= 1;
                    }
                    if y_value < end_point.y {
                        y_value += 1;
                    } else {
                        y_value -= 1;
                    }
                }

                let offset = self.vec_offset(end_point.x, end_point.y);
                self.coordinates.get_mut(offset).unwrap().mark();
            }
            // Part 1 doesn't care about diagonals, so there is no else
            return;
        }

        if start_point.x != end_point.x {
            let x_start = cmp::min(start_point.x, end_point.x);
            let x_end = cmp::max(start_point.x, end_point.x);

            for x_value in x_start..x_end + 1 {
                let offset = self.vec_offset(x_value, start_point.y);
                self.coordinates.get_mut(offset).unwrap().mark();
            }
        } else {
            let y_start = cmp::min(start_point.y, end_point.y);
            let y_end = cmp::max(start_point.y, end_point.y);

            for y_value in y_start..y_end + 1 {
                let offset = self.vec_offset(start_point.x, y_value);
                self.coordinates.get_mut(offset).unwrap().mark();
            }
        }
    }

    // The answer is the number of points where >= 2 points overlap
    fn result(&self) -> usize {
        self.coordinates
            .iter()
            .filter(|coord| coord.vent_count >= 2)
            .fold(0, |acc, _| acc + 1)
    }
}

#[derive(Debug)]
struct Coordinate {
    vent_count: usize,
}

impl Coordinate {
    fn new() -> Coordinate {
        Coordinate { vent_count: 0 }
    }

    fn mark(&mut self) {
        self.vent_count += 1
    }
}

fn main() {
    let parsed_input = parse_by_line(io::stdin().lock());

    let mut lines: Vec<Line> = Vec::new();

    for input_line in parsed_input {
        let mut split_line = input_line.split(" -> ");
        lines.push(Line::new(split_line.next(), split_line.next()));
    }

    let x_axis_max = lines.iter().fold(0, |acc, line| {
        cmp::max(cmp::max(acc, line.start.x), line.end.x)
    });
    let y_axis_max = lines.iter().fold(0, |acc, line| {
        cmp::max(cmp::max(acc, line.start.y), line.end.y)
    });

    // Initialize a map as a square
    // No downside to empty coordinates if one axis is smaller
    let mut map_part_1 = Map::new(cmp::max(x_axis_max, y_axis_max));
    let mut map_part_2 = Map::new(cmp::max(x_axis_max, y_axis_max));

    for line in lines.iter() {
        map_part_1.mark_line(line, false);
        map_part_2.mark_line(line, true);
    }
    println!("Part 1: {:?}", map_part_1.result());
    println!("Part 2: {:?}", map_part_2.result());
}
