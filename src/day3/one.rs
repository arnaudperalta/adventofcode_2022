use std::{fs::File, io::{BufReader, BufRead}};

pub fn please_work() {
    let file = File::open("./src/day3/input").unwrap();
    let mut priority = 0;
    for line in BufReader::new(file).lines() {
        let content = line.unwrap();
        if content.is_empty() {
            break;
        }
        let (first_comp, second_comp) = content.split_at(content.len() / 2);
        let mut please_break = false;
        for first_c in first_comp.chars() {
            for second_c in second_comp.chars() {
                if first_c == second_c {
                    if first_c.is_lowercase() {
                        priority += first_c as i32 - 96;
                    } else {
                        priority += first_c as i32 - 38;
                    }
                    please_break = true;
                    break;
                }
            }
            if please_break {
                break;
            }
        }
    }
    println!("{}", priority);
}
