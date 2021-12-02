use std::fs;
use day_02_rust::process_command_strings;

fn main() {
    let input_string = fs::read_to_string("input.txt").unwrap();

    let mut lines = vec![];

    input_string
        .split("\r\n")
        .for_each(|line| {
            lines.push(line);
        });

    let result = process_command_strings(lines);


    print!("{:?}\n", result);
    print!("{:?}\n", result.horizontal*result.depth);
}


