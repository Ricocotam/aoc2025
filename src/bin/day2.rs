use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];

    let content = fs::read_to_string(filepath)
        .expect("Couldn't read the file");

    let lines: Vec<&str> = content.trim().split("\n").collect();
    let commands: Vec<Command> = build_input(&lines);
    let result: u16 = compute_output(&commands);

    println!("Result is {}", result);
}

enum Command {
    L(u16),
    R(u16),
}

fn build_input(lines: &Vec<&str>) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();
    for line in lines.iter() {
        let (direction, step_str) = line.split_at(1);
        let step = step_str.parse::<u16>().expect("Failed parsing");
        let command: Command = match direction {
            "L" => Command::L(step),
            "R" => Command::R(step),
            &_ => panic!("{} is not a valid direction", direction),
        };
        commands.push(command);
    }

    commands
}

fn compute_output(commands: &Vec<Command>) -> u16 {
    let mut position: u16 = 50;
    let mut dial_pass: u16 = 0;

    for command in commands.iter() {
        let step: i32 = match command {
            Command::L(step_size) => (*step_size as i32) * -1,
            Command::R(step_size) => (*step_size).into(),
        };

        let temp = position as i32 + step;
        position = temp.rem_euclid(100).try_into().unwrap();

        dial_pass += (step.div_euclid(100)).try_into().unwrap();

        if position == 0 {
            dial_pass += 1;
        } else if temp > 100 && temp <= (100 + step) {
            dial_pass += 1;
        } else if temp < 0 && temp >= step {
            dial_pass += 1;
        }
    }

    dial_pass
}


