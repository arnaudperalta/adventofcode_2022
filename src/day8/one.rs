use std::fs::read_to_string;

pub fn please_work() {
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

    let mut visible = 0;
    for i in 0..height {
        for j in 0..width {
            if i == 0 || j == 0 || i == (height - 1) || j == (width - 1) {
                visible += 1;
                continue;
            }
            // Check from left
            let mut is_left_good = true;
            let tree_value = grid.get(i).unwrap().get(j).unwrap();
            for left in 0..j {
                if grid.get(i).unwrap().get(left).unwrap() >= tree_value {
                    is_left_good = false;
                    break;
                }
            }
            // Check from right
            let mut is_right_good = true;
            for right in (j + 1)..width {
                if grid.get(i).unwrap().get(right).unwrap() >= tree_value {
                    is_right_good = false;
                    break;
                }
            }
            // Check from top
            let mut is_top_good = true;
            for top in 0..i {
                if grid.get(top).unwrap().get(j).unwrap() >= tree_value {
                    is_top_good = false;
                    break;
                }
            }
            // Check from bottom
            let mut is_bottom_good = true;
            for bottom in (i + 1)..height {
                if grid.get(bottom).unwrap().get(j).unwrap() >= tree_value {
                    is_bottom_good = false;
                    break;
                }
            }

            if is_left_good || is_right_good || is_bottom_good || is_top_good {
                // println!("{} {} {} left {} right {} bottom {} top {}",
                //          grid.get(i).unwrap().get(j).unwrap(), i, j,
                //          is_left_good, is_right_good, is_bottom_good, is_top_good);
                visible += 1;
            }
        }
    }

    println!("{}", visible);
}

