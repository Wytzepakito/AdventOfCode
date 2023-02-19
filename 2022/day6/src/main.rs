use std::{fs, collections::HashSet};




fn main() {
    let binding = fs::read_to_string("input.txt").expect("Could not read input file");
    let mut lines = binding.lines();

    for line in lines {
        println!("{}", line);
        for i in 14..line.len() {
            let mut slice = &line[i-14..i];

            let mut set = HashSet::new();

            for ch in slice.chars() {
                set.insert(ch);
            }
            if set.len() == 14 {
                println!("First marker found after charcter {}", i);
                break;
            }
        }
    }
}
