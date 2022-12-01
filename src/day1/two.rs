use std::{fs::File, io::{BufReader, BufRead}};

pub fn please_work_again() {
    let file = File::open("./src/day1/input").unwrap();
    let mut actual_sum = 0;
    let mut vec = vec![];
    for line in BufReader::new(file).lines() {
        if let Ok(content) = line {
            if content.is_empty() {
                vec.push(actual_sum);
                actual_sum = 0;
            } else {
                actual_sum += content.parse::<i32>().unwrap();
            }
        }
    }
    vec.sort();
    let total =
        vec[vec.len() - 1] 
        + vec[vec.len() - 2]
        + vec[vec.len() - 3];
    println!("{}", total);
}
