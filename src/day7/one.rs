use std::{fs::read_to_string, io::BufRead, rc::Rc, cell::RefCell, borrow::Borrow};

struct TreeNode {
    folder_name: String,
    folder_size: usize,
    parent: Option<Rc<RefCell<TreeNode>>>,
    children: Vec<Rc<RefCell<TreeNode>>> // only folders are children
}

pub fn please_work() {
    let text = read_to_string("./src/day7/input").unwrap();
    let root = Rc::new(RefCell::new(TreeNode {
        folder_name: "/".to_string(),
        folder_size: 0,
        parent: None,
        children: vec![]
    }));
    let mut actual_node = Rc::clone(&root);
    for line in text.lines() {
        if line.contains("cd /") {
            continue;
        }
        if line.contains("cd ") {
            let node_to_switch: RefCell<TreeNode>;
            let folder_name = line.split("$ cd ").nth(1).unwrap();
            for node in actual_node.children {
                if node.borrow() == folder_name {
                    actual_node = node.borrow();
                    break;
                }
            }
        }
        if line.contains("dir ") {
            let new_child = Rc::new(RefCell::new(TreeNode {
                folder_name: line.split("dir ").nth(1).unwrap().to_string(),
                folder_size: 0,
                parent: Some(actual_node),
                children: vec![]
            }));
            actual_node.children.push(Rc::new(RefCell::new(new_child)));
            continue;
        }
        if !line.contains("$") {
            let file_size = line.split(" ").nth(0).unwrap().parse::<usize>().unwrap();
            println!("{}", file_size);
        }
    }
}
