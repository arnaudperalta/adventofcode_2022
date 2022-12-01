use std::{fs::File, io::{BufReader, BufRead}};

pub fn please_work() {
    let file = File::open("./src/day1/input").unwrap();
    let mut max_sum = 0;
    let mut actual_sum = 0;
    for line in BufReader::new(file).lines() {
        if let Ok(content) = line {
            if content.is_empty() {
                if max_sum < actual_sum {
                    max_sum = actual_sum;
                }
                actual_sum = 0;
            } else {
                actual_sum += content.parse::<i32>().unwrap();
            }
        }
    }
    println!("{}", max_sum);
}
