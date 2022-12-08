use std::{fs::read_to_string, rc::Rc, cell::RefCell};

struct TreeNode {
    folder_name: String,
    folder_size: usize,
    parent: Option<Rc<RefCell<TreeNode>>>,
    children: Vec<Rc<RefCell<TreeNode>>>
}

pub fn please_work_again() {
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
        else if line.contains("cd ..") {
            let temp = actual_node.borrow_mut().parent.clone().unwrap();
            actual_node = temp;
        }
        else if line.contains("cd ") {
            let folder_name = line.split("$ cd ").nth(1).unwrap();
            let mut to_change = Rc::new(RefCell::new(TreeNode {
                folder_name: "bug".to_string(),
                folder_size: 0,
                parent: None,
                children: vec![]
            }));

            for node in actual_node.borrow_mut().children.iter() {
                if node.borrow_mut().folder_name == folder_name {
                    to_change = node.clone();
                    break;
                }
            }
            actual_node = to_change;
        }
        else if line.contains("dir ") {
            let new_child = Rc::new(RefCell::new(TreeNode {
                folder_name: line.split("dir ").nth(1).unwrap().to_string(),
                folder_size: 0,
                parent: Some(actual_node.to_owned()),
                children: vec![]
            }));
            actual_node.borrow_mut().children.push(new_child);
            continue;
        }
        else if !line.contains("$") {
            let file_size = line.split(" ").nth(0).unwrap().parse::<usize>().unwrap();
            actual_node.borrow_mut().folder_size += file_size;
            let mut parent = actual_node.borrow().parent.clone();
            while parent.clone().is_some() {
                let to_change = parent.clone(); 
                to_change.as_ref().unwrap().borrow_mut().folder_size += file_size;
                parent = to_change.as_ref().unwrap().borrow_mut().parent.clone();
            }
        }
    }
    let mut stack_sizes = vec![];
    let base_size = root.borrow().folder_size;
    get_sizes(root, &mut stack_sizes, base_size);
    stack_sizes.sort();
    println!("{}", stack_sizes.first().unwrap());
}

fn get_sizes(tree: Rc<RefCell<TreeNode>>, sizes: &mut Vec<usize>, base_size: usize) {
    if 70000000 - base_size + tree.borrow_mut().folder_size > 30000000 {
        let wesh = tree.borrow();
        println!("name {}, size {}", wesh.folder_name, wesh.folder_size);
        sizes.push(wesh.folder_size);
    }
    for node in tree.borrow_mut().children.iter() {
        get_sizes(node.to_owned(), sizes, base_size);
    }
}
