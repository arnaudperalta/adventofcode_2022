use std::{fs::File, io::{BufReader, BufRead}};

pub fn please_work_again() {
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
                    'X' => score += 0 + 3, // Lose => Z
                    'Y' => score += 3 + 1, // Draw => X
                    'Z' => score += 6 + 2, // Win => Y
                    _ => ()
                }
            },
            'B' => { // Paper
                match my_choice {
                    'X' => score += 0 + 1, // Lose => X
                    'Y' => score += 3 + 2, // Draw => Y
                    'Z' => score += 6 + 3, // Win => Z
                    _ => ()
                }
            },
            'C' => { // Scissors
                match my_choice {
                    'X' => score += 0 + 2, // Lose => Y
                    'Y' => score += 3 + 3, // Draw => Z
                    'Z' => score += 6 + 1, // Win => X
                    _ => ()
                }
            },
            _ => ()
        }
    }
    println!("{}", score);
}
