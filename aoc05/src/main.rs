use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_stack_line(line: &str) -> Vec<Option<u8>> {
    assert!(line.is_ascii());
    let mut result = Vec::new();
    for i in (0..line.len()).step_by(4) {
        let substring = &line[i..i + 3];
        if substring == "   " {
            result.push(None);
            continue;
        }
        let substring = substring.as_bytes();
        assert!(substring[0] == b'[' && substring[2] == b']');
        result.push(Some(substring[1]));
    }
    result
}

struct Command {
    amount: usize,
    from: usize,
    to: usize,
}

enum Part {
    Part1,
    Part2,
}

fn execute_commands(stacks: &mut [Vec<u8>], commands: &Vec<Command>, part: Part) {
    for command in commands {
        assert!(command.from != command.to);
        let source_stack = &mut stacks[command.from - 1];
        let mut copy_buffer = Vec::with_capacity(command.amount);
        for current_crate in &source_stack[source_stack.len() - command.amount..] {
            copy_buffer.push(*current_crate);
        }
        source_stack.drain(source_stack.len() - command.amount..);

        let target_stack = &mut stacks[command.to - 1];

        match part {
            Part::Part1 => {
                for current_crate in copy_buffer.iter().rev() {
                    target_stack.push(*current_crate);
                }
            }
            Part::Part2 => {
                for current_crate in copy_buffer {
                    target_stack.push(current_crate);
                }
            }
        }
    }
}

fn main() {
    let file = File::open("real_input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut parsed_lines = Vec::new();
    let mut commands = Vec::new();

    let mut command_section = false;
    for line in reader.lines().map(|line| line.unwrap()) {
        if !command_section && (b'0'..b'9').contains(&line.as_bytes()[1]) {
            command_section = true;
        }
        if !command_section {
            parsed_lines.push(parse_stack_line(&line));
        } else if !line.is_empty() {
            let parts: Vec<_> = line.split(' ').map(str::parse).collect();
            if let [_, Ok(amount), _, Ok(from), _, Ok(to)] = parts[..] {
                commands.push(Command { amount, from, to });
            }
        }
    }
    let num_stacks = parsed_lines.iter().map(|line| line.len()).max().unwrap();

    let mut stacks = vec![Vec::<u8>::new(); num_stacks];
    for line in parsed_lines.iter().rev() {
        for (stack_index, current_crate) in line.iter().enumerate() {
            if let Some(current_crate) = current_crate {
                stacks[stack_index].push(*current_crate);
            }
        }
    }

    execute_commands(&mut stacks, &commands, Part::Part2);

    for stack in &stacks {
        print!(
            "{}",
            stack
                .iter()
                .last()
                .map(|current_crate| *current_crate as char)
                .unwrap()
        );
    }
}
