#[cfg(test)]
mod tests {
    use day_02_rust::{build_command, Position, process_command_strings, process_commands};
    use day_02_rust::Command;

    #[test]
    fn forward_0_gives_forward() {
        let result = build_command("forward 0");

        assert_eq!(Command::Forward(0), result);
    }

    #[test]
    fn command_down_1_gives_position_depth_1() {
        let command = build_command("down 1");

        let position = process_commands(vec![command]);
        let expected_position = Position {
            horizontal: 0,
            depth: 0,
            aim: 1
        };

        assert_eq!(expected_position, position);
    }

    #[test]
    fn two_command_up_1_forward_1_gives_position_depth_m1_horiz_1() {
        let command1 = build_command("up 1");
        let command2 = build_command("forward 1");

        let position = process_commands(vec![command1, command2]);
        let expected_position = Position {
            horizontal: 1,
            depth: -1,
            aim: -1
        };

        assert_eq!(expected_position, position);
    }

    #[test]
    fn example_gives_position_15_10() {
        let position = process_command_strings(vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]);

        assert_eq!(15, position.horizontal);
        assert_eq!(60, position.depth);
    }
}