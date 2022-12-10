use std::fs::read_to_string;

pub fn please_work_again() {
    let text = read_to_string("./src/day8/input").unwrap();
    let mut grid = vec![];
    let mut width = 0;
    let mut height = 0;
    for (i, line) in text.lines().enumerate() {
        height = i + 1;
        grid.push(vec![]);
        for (j, c) in line.chars().enumerate() {
            width = j + 1;
            if c == '\n' {
                break;
            }
            grid.get_mut(i).unwrap().push(c);
        }
    }

    let mut best_score = 0;
    for i in 0..height {
        for j in 0..width {
            if i == 0 || j == 0 || i == (height - 1) || j == (width - 1) {
                continue;
            }
            let tree_value = grid.get(i).unwrap().get(j).unwrap().to_digit(10).unwrap();

            // Check from left
            let mut left_score = 0;
            for left in (0..j).rev() {
                left_score += 1;
                if grid.get(i).unwrap().get(left).unwrap().to_digit(10).unwrap() >= tree_value {
                    break;
                }
            }
            // Check from right
            let mut right_score = 0;
            for right in j + 1 .. width {
                right_score += 1;
                if grid.get(i).unwrap().get(right).unwrap().to_digit(10).unwrap() >= tree_value {
                    break;
                }
            }

            // Check from top
            let mut top_score = 0;
            for top in (0..i).rev() {
                top_score += 1;
                if grid.get(top).unwrap().get(j).unwrap().to_digit(10).unwrap() >= tree_value {
                    break;
                }
            }
            // Check from bottom
            let mut bottom_score = 0;
            for bottom in i + 1 .. height {
                bottom_score += 1;
                if grid.get(bottom).unwrap().get(j).unwrap().to_digit(10).unwrap() >= tree_value {
                    break;
                }
            }
            let actual_score = top_score * bottom_score * right_score * left_score;
            if actual_score > best_score {
                best_score = actual_score;
            }
            // println!("actual {} top {} bot {} right {} left {} {} {} {}", actual_score, top_score, bottom_score, right_score, left_score, i, j, tree_value);
        }
    }
    println!("{}", best_score);
}

