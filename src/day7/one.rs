use std::{fs::{File, read_to_string}, io::{BufReader, BufRead}};

pub fn please_work() {
    let text = read_to_string("./src/day7/input").unwrap();
    let mut size_stack = Vec::<usize>::new();
    for (i, line) in text.lines().enumerate() {
        if line.is_empty() {
            break;
        }
        if i < 2 {
            continue;
        }
        if line.contains("dir") {
            let dir_name = line.split("dir ").nth(1).unwrap();
            get_directory_size(text.clone(), dir_name, &mut size_stack);
        }
    }
    let mut total = 0;
    for size in size_stack.iter() {
        println!("{}", size);
        total += size;
    }
    println!("TOTAL {}", total);
}

fn get_directory_size(text: String, name: &str, sizes: &mut Vec<usize>) -> usize {
    let mut skip_lines = -1;
    let command = "$ cd ".to_owned() + name;
    let mut size = 0;
    for line in text.lines() {
        if line.contains(command.as_str()) {
            skip_lines = 2;
        }
        if skip_lines > 0 {
            skip_lines -= 1;
            continue;
        }
        if skip_lines == 0 {
            if line.contains("dir ") {
                let dir_name = line.split("dir ").nth(1).unwrap();
                size += get_directory_size(text.clone(), dir_name, sizes);
            } else if line.contains("$") {
                break;
            } else {
                let file_size = line.split(" ").next().unwrap().parse::<usize>();
                match file_size {
                    Ok(v) => size += v,
                    Err(_) => ()
                }
            }
        }
    }
    if size < 100000 {
        if !(sizes.iter().any(|&i| i == size)) {
            println!("{}", size);
            sizes.push(size.to_owned());
        }
    }
    return size;
}
