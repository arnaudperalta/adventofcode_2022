use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

pub fn please_work_again() {
    let file = File::open("./src/day3/input").unwrap();
    let mut priority = 0;
    let mut line_of_group = 1;
    let mut occ = HashMap::new();
    for line in BufReader::new(file).lines() {
        let content = line.unwrap();
        let mut actual_occ = HashMap::new();
        if content.is_empty() {
            break;
        }
        for c in content.chars() {
            match actual_occ.get(&c) {
                None => {
                    actual_occ.insert(c, 1);
                    match occ.get(&c) {
                        Some(res) => occ.insert(c, (res + 1) % (line_of_group + 1)),
                        None => occ.insert(c, 1)
                    };
                }
                _ => ()
            }
        }
        if line_of_group == 3 { // Si on a lu 3 lignes
            for (key, value) in occ.clone().into_iter() {
                println!("{} {}", key, value);
                if value == 3 {
                    if key.is_lowercase() {
                        priority += key as i32 - 96;
                    } else {
                        priority += key as i32 - 38;
                    }
                }
            }
            occ.clear();
        }
        line_of_group = (line_of_group % 3) + 1;
    }
    println!("{}", priority);
}
