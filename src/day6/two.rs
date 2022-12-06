use std::{fs::File, io::{BufReader, BufRead}, collections::VecDeque};

pub fn please_work_again() {
    let file = File::open("./src/day6/input").unwrap();
    let mut buffer = VecDeque::<char>::new();
    for line in BufReader::new(file).lines() {
        let content = line.unwrap();
        if content.is_empty() {
            break;
        }
        for (i, c) in content.chars().enumerate() {
            if buffer.len() >= 13 {
                if !(buffer.iter().any(|&i| i == c)) {
                    let mut is_bad = false;
                    for (i, element) in buffer.iter().enumerate() {
                        for (j, second_element) in buffer.iter().enumerate() {
                            if i != j && *element == *second_element {
                                is_bad = true;
                                break;
                            }
                        }
                    }
                    if !is_bad {
                        println!("{}", i + 1);
                        return;
                    }
                }
                buffer.pop_back();
            }
            buffer.push_front(c);
        }
    }
}
