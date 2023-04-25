#![allow(dead_code, unused_mut, unused_assignments)]
use std::fs::File;
use std::io::Read;

pub struct Tree {
    num_of_children : i32,
    string : String,
    flag : bool,
    father : *mut Tree,
    child : *mut Tree,
    next : *mut Tree
}

// read a file and return a copy in string
pub fn fstring(path: &str) -> Option<String> {
    let mut file = File::open(path).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    Some(contents)
}

// return true if string is numerical
pub fn numerical_string(s: &str) -> bool{
    if s.is_empty() {
        return false;
    }

    for c in s.chars() {
        if !c.is_ascii_digit() {
            return false;
        }
    }

    true
}

// returns a tree with the value of string
pub fn do_tree(c: &str) -> Box<Tree> {
    let l = Box::new(Tree {
        num_of_children: 0,
        string: c.to_string(),
        flag: numerical_string(c),
        father: std::ptr::null_mut(),
        child: std::ptr::null_mut(),
        next: std::ptr::null_mut()
    });
    l
}

// adds a brother to the tree, incase the brother already exists,
// goes to the bottom and adds the brother 
pub fn add_next(mut t: *mut Tree, mut brother: *mut Tree) {
    if brother.is_null() {
        return;
    }

    if t.is_null() {
        t = brother;
        return;
    }
    
    let ref mut temp1 = unsafe { brother.read().father };
    *temp1 = unsafe { t.read().father };
    
    if !(unsafe { t.read().father }).is_null() {
        unsafe { brother.read().father.read() }.num_of_children += 1;
    }
    
    while !(unsafe { t.read().next }).is_null() {
        t = unsafe { t.read().next };
    }
    
    let ref mut temp2 = unsafe { t.read().next };
    *temp2 = brother;
}


