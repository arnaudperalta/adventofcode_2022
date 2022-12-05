use std::{fs::File, io::{BufReader, BufRead}};

pub fn please_work_again() {
    let file = File::open("./src/day4/input").unwrap();
    let mut full_contains = 0;
    for line in BufReader::new(file).lines() {
        let content = line.unwrap();
        if content.is_empty() {
            break;
        }
        let mut sequences = content.split(",");
        let first_seq = sequences.next().unwrap();
        let second_seq = sequences.next().unwrap();
        let mut first_split = first_seq.split("-");
        let mut second_split = second_seq.split("-");
        let numbers: [usize; 4] = [
            first_split.next().unwrap().parse().unwrap(),
            first_split.next().unwrap().parse().unwrap(),
            second_split.next().unwrap().parse().unwrap(),
            second_split.next().unwrap().parse().unwrap()
        ];
        
        if numbers[0] <= numbers[2] && numbers[1] >= numbers[2] {
            full_contains += 1;
        } else if numbers[2] <= numbers[0] && numbers[3] >= numbers[0] {
            full_contains += 1;
        }
    }
    println!("{}", full_contains);
}
