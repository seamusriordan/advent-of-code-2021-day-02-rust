#[derive(PartialEq, Debug)]
pub enum Command {
    Up(u32),
    Down(u32),
    Forward(u32),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Position {
    pub horizontal: i32,
    pub depth: i32,
    pub aim: i32
}

pub fn process_command_strings(command_strings: Vec<&str>) -> Position {
    let commands = command_strings.iter().map(
        |&command_string| {
            build_command(command_string)
        }
    ).collect();
    process_commands(commands)
}

pub fn build_command(command_string: &str) -> Command {
    let mut parts = command_string.split(" ");

    let command_part = parts.next().unwrap();
    let size = parts.next().unwrap().parse::<u32>().unwrap();

    return match command_part {
        "forward" => Command::Forward(size),
        "up" => Command::Up(size),
        "down" => Command::Down(size),
        _ => { panic!() }
    };
}

pub fn process_commands(commands: Vec<Command>) -> Position {
    let mut acc = Position { horizontal: 0, depth: 0, aim: 0 };

    for command in commands {
        match command {
            Command::Down(n) => { acc.aim += n as i32 }
            Command::Up(n) => { acc.aim -= n as i32 }
            Command::Forward(n) => {
                acc.horizontal += n as i32;
                acc.depth += (n as i32)*acc.aim;
            }
        }
    }

    acc
}