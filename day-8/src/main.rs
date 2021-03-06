//use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
struct Instruction {
    op: String,
    arg: i32,
}

fn main() {
    let file = fs::read_to_string("./data.txt").expect("Error reading file");
    let instructions = file
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            let op = parts[0];
            let arg = parts[1];
            if arg.starts_with("+") {
                let arg = arg.strip_prefix("+");
            }
            let arg = arg.parse::<i32>().expect("Invalid integer");
            Instruction {
                op: op.to_string(),
                arg: arg,
            }
        })
        .collect();

    problem1(&instructions);
    problem2(&instructions);
}

fn problem1(instructions: &Vec<Instruction>) {
    println!("Problem 1");

    let mut position = 0;
    let mut accumulator = 0;
    let mut visited = vec![false; instructions.len()];
    loop {
        if visited[position] == true {
            break;
        }
        visited[position] = true;

        let instruction = &instructions[position];
        match instruction.op.as_str() {
            "acc" => {
                accumulator += instruction.arg;
                position += 1;
            }
            "jmp" => position = ((position as i32) + instruction.arg) as usize,
            "nop" => position += 1,
            _ => {
                panic!("Invalid operation: {}", instruction.op);
            }
        }
    }

    println!("Accumulator: {}", accumulator);
}

fn problem2(instructions: &Vec<Instruction>) {
    println!("Problem 2");

    for (i, instruction) in instructions.iter().enumerate() {
        let replacement = match instruction.op.as_str() {
            "acc" => continue,
            "jmp" => Instruction {
                op: "nop".to_string(),
                arg: instruction.arg,
            },
            "nop" => Instruction {
                op: "jmp".to_string(),
                arg: instruction.arg,
            },
            _ => continue,
        };

        let mut replaced = instructions.to_vec();
        replaced[i] = replacement;
        let (accumulator, finished) = helper(&replaced);
        if finished {
            println!("Accumulator: {}", accumulator);
            return;
        }
    }
}

fn helper(instructions: &Vec<Instruction>) -> (i32, bool) {
    let mut position = 0;
    let mut accumulator = 0;
    let mut visited = vec![false; instructions.len()];
    loop {
        if position == instructions.len() {
            return (accumulator, true);
        }
        if visited[position] == true {
            return (0, false);
        }
        visited[position] = true;

        let instruction = &instructions[position];
        match instruction.op.as_str() {
            "acc" => {
                accumulator += instruction.arg;
                position += 1;
            }
            "jmp" => position = ((position as i32) + instruction.arg) as usize,
            "nop" => position += 1,
            _ => {
                panic!("Invalid operation: {}", instruction.op);
            }
        }
    }
}
