use std::{fs::File, io::{BufReader, BufRead}, collections::VecDeque};

pub fn please_work() {
    let file = File::open("./src/day6/input").unwrap();
    let mut buffer = VecDeque::<char>::new();
    for line in BufReader::new(file).lines() {
        let content = line.unwrap();
        if content.is_empty() {
            break;
        }
        for (i, c) in content.chars().enumerate() {
            if buffer.len() >= 3 {
                if buffer.iter().any(|&i| i == c) {
                    buffer.pop_back();
                } else {
                    if !(buffer.get(0).unwrap() == buffer.get(1).unwrap())
                            && !(buffer.get(0).unwrap() == buffer.get(2).unwrap())
                            && !(buffer.get(1).unwrap() == buffer.get(2).unwrap()) {
                        println!("{}", i + 1);
                        return;
                    }
                }
            }
            buffer.push_front(c);
        }
    }
}
