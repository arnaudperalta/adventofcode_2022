use std::{fs::File, io::{BufReader, BufRead}};

pub fn please_work_again() {
    let file = File::open("./src/day5/input").unwrap();
    let buffer = BufReader::new(file).lines();
    let mut stack_count = 0;
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut now_instructions = false;
    let mut aux_stack = Vec::new();
    for line in buffer {
        let content = line.unwrap();
        if now_instructions {
            let splits = content.split(" ").collect::<Vec<&str>>();
            let number_of_move = splits[1].parse::<usize>().unwrap();
            let from = splits[3].parse::<usize>().unwrap() - 1;
            let to = splits[5].parse::<usize>().unwrap() - 1;

            for _ in 0..number_of_move {
                let value = stacks.get_mut(from).unwrap().pop().unwrap();
                aux_stack.push(value);
            }
            for _ in 0..number_of_move {
                let value = aux_stack.pop().unwrap();
                stacks.get_mut(to).unwrap().push(value);
            }
        }
        if content.is_empty() {
            now_instructions = true;
        }
        if stack_count == 0 {
            stack_count = content.len() / 4 + 1;
            for _ in 0..stack_count {
                stacks.push(Vec::new());
            }
        }
        let mut next_is_crate = false;
        for (i, c) in content.chars().enumerate() {
            if next_is_crate {
                let actual_stack = i / 4;
                stacks.get_mut(actual_stack).unwrap().insert(0, c);
                next_is_crate = false;
            }
            if c == '[' {
                next_is_crate = true;
            }
        }
    }
    for i in 0..stack_count {
        let value = stacks.get_mut(i).unwrap().pop().unwrap();
        print!("{}", value);
    }
    print!("\n");
}
