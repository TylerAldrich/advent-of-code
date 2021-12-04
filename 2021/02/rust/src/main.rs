use advent_input_parser::parse_by_line;
use std::io;

#[derive(Debug)]
enum Command {
    Forward(isize),
    Down(isize),
    Up(isize),
}

// I like to think the submarine is piloted by cats
#[derive(Debug)]
struct Meowmarine {
    horizontal: isize,
    depth: isize,
    aim: isize,
}

impl Meowmarine {
    fn new() -> Meowmarine {
        Meowmarine {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    // Used for part 1 of the puzzle
    fn command_1(self, command: &Command) -> Meowmarine {
        match command {
            Command::Forward(amt) => Meowmarine {
                horizontal: self.horizontal + amt,
                ..self
            },
            // Note that Down means depth increases
            Command::Down(amt) => Meowmarine {
                depth: self.depth + amt,
                ..self
            },
            Command::Up(amt) => Meowmarine {
                depth: self.depth - amt,
                ..self
            },
        }
    }

    // Used for part 2 of the puzzle.
    fn command_2(self, command: &Command) -> Meowmarine {
        match command {
            Command::Forward(amt) => Meowmarine {
                horizontal: self.horizontal + amt,
                depth: self.depth + self.aim * amt,
                ..self
            },
            // Note that Down means aim increases
            Command::Down(amt) => Meowmarine {
                aim: self.aim + amt,
                ..self
            },
            Command::Up(amt) => Meowmarine {
                aim: self.aim - amt,
                ..self
            },
        }
    }

    // Value used for the solution.
    fn multiplied(&self) -> isize {
        self.depth * self.horizontal
    }
}

fn main() {
    let parsed_input = parse_by_line(io::stdin().lock());

    let mut commands: Vec<Command> = Vec::new();
    for value in parsed_input {
        let split_value: Vec<&str> = value.split(" ").collect();
        if split_value.len() != 2 {
            panic!(
                "Invalid input. Commands must be of the form:\nCommandName Value\ne.g. 'forward 2'"
            )
        }

        let command_name = split_value.get(0).unwrap();
        let command_value = split_value
            .get(1)
            .unwrap()
            .parse::<isize>()
            .expect("Value must be an integer.");

        match *command_name {
            "forward" => commands.push(Command::Forward(command_value)),
            "down" => commands.push(Command::Down(command_value)),
            "up" => commands.push(Command::Up(command_value)),
            _ => panic!("Invalid command. Valid commands are: 'up', 'down', 'forward"),
        };
    }

    let mut meowmarine = Meowmarine::new();
    for command in &commands {
        meowmarine = meowmarine.command_1(command);
    }

    println!(
        "Part 1 final position: {:?}, multiplied = {:?}",
        meowmarine,
        meowmarine.multiplied()
    );

    let mut meowmarine_mk2 = Meowmarine::new();
    for command in &commands {
        meowmarine_mk2 = meowmarine_mk2.command_2(command);
    }

    println!(
        "Part 2 final position: {:?}, multiplied = {:?}",
        meowmarine_mk2,
        meowmarine_mk2.multiplied()
    );
}
