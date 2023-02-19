use std::{fs, collections::HashMap, ops::Index};
use lazy_static::lazy_static;
use regex::Regex;


fn make_stacks(crate_lines: &Vec<&str>, index_line: &str) -> HashMap<String, Vec<String>> {
    let mut stacks: HashMap<String, Vec<String>> = HashMap::new();
    let mut stack_names: Vec<&str> = index_line.split_whitespace().collect();

    for line in crate_lines.iter().rev() {

        for stack_name in &stack_names {
            let index = index_line.chars().position(|c| c == stack_name.chars().next().unwrap()).unwrap();
            let vec: Vec<char> = (**line).chars().collect();
            let possible_crate = vec[index];

            if possible_crate.is_alphabetic() {
                let mut stack = stacks.entry(stack_name.to_string()).or_default();
                stack.push(possible_crate.to_string());

            }

        }
    }

    println!("Stacks are: {:?}", stacks);
    stacks
}

fn parse_move_line(line: &str) -> (String, String, String) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }

    let result: Vec<&str> = RE.find_iter(line).map( |m| m.as_str()).collect();
    println!("{:?}", result);
    return (result[0].to_string(), result[1].to_string(), result[2].to_string());
}



fn read_file(filename: &str) -> StackPuzzle {
    let mut file_string = fs::read_to_string(filename).expect("I could not read this file");
    let mut stacks: HashMap<String, Vec<String>> = HashMap::new();
    let mut made_stack = false;

    let mut crate_lines: Vec<&str> = Vec::new();
    let mut moves: Vec<(String, String, String)> = Vec::new();


    for line in file_string.lines() {
        if !line.contains("[") && made_stack == false {
            stacks = make_stacks(&crate_lines, line);
            made_stack = true;
        } else if made_stack == true && line != "" {
            moves.push(parse_move_line(line));
        }

        if made_stack == false {
            crate_lines.push(line);
        }


    }

     let mut stackpuzzle =StackPuzzle {
        moves: moves,
        stacks: stacks
    };
    stackpuzzle
    
}

struct StackPuzzle {
    moves: Vec<(String, String, String)>,
    stacks: HashMap<String, Vec<String>>
}


fn solve_puzzle(mut stackpuzzle: StackPuzzle) {

    for one_move in stackpuzzle.moves {
        let mut source = stackpuzzle.stacks.get_mut(&one_move.1).expect("Could not find key of source");
        
        let new_length = source.len().saturating_sub(one_move.0.parse().unwrap());
        let tail = source.split_off(new_length);

        let mut destinations = stackpuzzle.stacks.get_mut(&one_move.2).expect("Could not find key destination");
        destinations.extend(tail);
    }

    println!("stacks: {:?}",  stackpuzzle.stacks);
    println!("The word being made by all top boxes is:");
    for i in 1..stackpuzzle.stacks.len() + 1 {
        let stack = stackpuzzle.stacks.get(&i.to_string()).expect("Could not find stack name in stacks");
        print!("{}", stack.last().expect("Stack was empty"));
    }
    println!("");        
}


fn main() {
    println!("Hello, world!");

    let mut stackpuzzle = read_file("input.txt");
    solve_puzzle(stackpuzzle)
}
