use std::{fs::File, io::{BufReader, BufRead}};

pub fn please_work() {
    let file = File::open("./src/day2/input").unwrap();
    let mut score = 0;
    for line in BufReader::new(file).lines() {
        let content = line.unwrap();
        if content.is_empty() {
            break;
        }
        let ennemy_choice = content.as_bytes()[0] as char;
        let my_choice = content.as_bytes()[2] as char;
        match ennemy_choice {
            'A' => { // Rock
                match my_choice {
                    'X' => score += 3 + 1, // Rock
                    'Y' => score += 6 + 2, // Paper
                    'Z' => score += 0 + 3, // Scissors
                    _ => ()
                }
            },
            'B' => { // Paper
                match my_choice {
                    'X' => score += 0 + 1, // Rock
                    'Y' => score += 3 + 2, // Paper
                    'Z' => score += 6 + 3, // Scissors
                    _ => ()
                }
            },
            'C' => { // Scissors
                match my_choice {
                    'X' => score += 6 + 1, // Rock
                    'Y' => score += 0 + 2, // Paper
                    'Z' => score += 3 + 3, // Scissors
                    _ => ()
                }
            },
            _ => ()
        }
    }
    println!("{}", score);
}
