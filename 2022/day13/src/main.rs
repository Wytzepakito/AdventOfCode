use std::{fs, thread, time};
use regex::Regex;

fn read_file(filename: &str) -> Vec<(String, String)> {
    let mut file_string = fs::read_to_string(filename).expect("Couldn't read file...");
    let mut packets: Vec<(String, String)> = Vec::new();

    let mut line_cache: Vec<&str> = Vec::new();
    for line in file_string.lines() {
        if line.is_empty() {
            packets.push((line_cache[0].to_string(), line_cache[1].to_string()));
            line_cache = Vec::new();
        } else {
            line_cache.push(line);
        }
    }
    packets.push((line_cache[0].to_string(), line_cache[1].to_string()));
    packets
}

fn insert_list(mut string: String, index: usize) -> String {
    let mut list_end = string.len();
    for (i, ch) in string.chars().skip(index).enumerate() {
        if !ch.is_digit(10) && ch != ',' {
            list_end = i + index + 1;
        }
    }

    string.insert(index, '[');
    string.insert(list_end, ']');

    return (string);
}

fn solve_mixed_types(packet: &  (String, String)) -> (String, String) {
    let mut ended = false;
    let mut string1 = packet.0.clone();
    let mut string2 = packet.1.clone();
    let mut s1_ind = 0;
    let mut s2_ind = 0;

    while (ended == false) {
        let mut broke = "";
        let mut index = 0;

        while (s1_ind != string1.len() && s2_ind != string2.len()) {
            let s1_ch = string1.chars().nth(s1_ind).unwrap();
            let s2_ch = string2.chars().nth(s2_ind).unwrap();
            if (s1_ch == s2_ch)
                || (s1_ch.is_digit(10) && s2_ch.is_digit(10))
            {
                s1_ind += 1;
                s2_ind += 1;
                continue;
            } else if (s1_ch.is_digit(10) || s1_ch == ']' ) && s2_ch == '[' {
                index = s1_ind;
                broke = "string1";
                break;
            } else if s1_ch == '[' && (s2_ch.is_digit(10) || s2_ch == ']') {
                index = s1_ind;
                broke = "string2";
                break;
            } else if s1_ch == ']' && (s2_ch.is_digit(10) || s2_ch == ',') {
                s2_ind += 1;
            } else if (s1_ch.is_digit(10) || s1_ch == ',') && s2_ch == ']' {
                s1_ind += 1;
            }
        }
        if !broke.is_empty() {
            match broke {
                "string1" => string1 = insert_list(string1, index),
                "string2" => string2 = insert_list(string2, index),
                _ => unreachable!(),
            }
            broke = "";
        } else if broke.is_empty() {
            ended = true;
        }
    }
    return ((string1, string2));
    
}




fn get_next_list(string: &str, index: usize) -> (usize, Vec<u8>) {
    let mut index_copy = index;
    while index_copy != string.len() {
        let s_ch = string.chars().nth(index_copy).unwrap(); 
        if s_ch == ']' {
            // stop
            break;
        }
        index_copy += 1;
    }

    let regex = Regex::new(r"\d+").unwrap();
    let new_string = string[index..index_copy].to_string();
    let numbers: Vec<u8> = regex.find_iter(&new_string).filter_map(|digits| digits.as_str().parse().ok()).collect();

    println!("{:?}", string[index..index_copy].to_string());
    return (index_copy, numbers);
}

fn is_list_winner(string1: &str, s1_ind: usize, string2: &str, s2_ind: usize) -> (bool, usize, usize) {

    let (new_s1_ind, s1_numbers) = get_next_list(string1, s1_ind);
    let (new_s2_ind, s2_numbers) = get_next_list(string2, s2_ind);

    println!("{:?}, {:?}", s1_numbers, s2_numbers);

    for (num1, num2) in s1_numbers.iter().zip(s2_numbers.iter()) {
        if num1 == num2 {
            continue;
        } else if num1 > num2 {
            println!("Integers were not in the correct orders");
            return(true, 0, 0);
        } else {
            println!("Integers were in the correct order");
            return(true, 0, 0);
        }
    }

    if s1_numbers.len() < s2_numbers.len() {
        println!("Integers were in the correct order");
        return(true, 0, 0);
    } else if s1_numbers.len() > s2_numbers.len() {
        println!("Integers were not in the correct order");
        return(true, 0, 0);
    } else {
        return(false, new_s1_ind, new_s2_ind);
    }
}

fn check_ordering(string1: String, string2: String, index: usize) {

    let mut s1_ind = 0;
    let mut s2_ind = 0;

    while(s1_ind != string1.len() && s2_ind != string2.len()) {
        let s1_ch = string1.chars().nth(s1_ind).unwrap();
        let s2_ch = string2.chars().nth(s2_ind).unwrap();

        println!("s1_ind: {}, s1_ch: {}, s2_ind: {}, s2_ch: {}", s1_ind, s1_ch, s2_ind, s2_ch);
        if (s1_ch == '[' && s2_ch == '[') {
            s1_ind +=1;
            s2_ind +=1;
        } else if (s1_ch == ',' && s2_ch == ',') {
            s1_ind +=1;
            s2_ind +=1;
        } else if (s1_ch.is_digit(10) && s2_ch.is_digit(10)) {
            // Get next list
            let ( win, mut new_s1_ind, mut new_s2_ind) = is_list_winner(&string1, s1_ind, &string2, s2_ind);
            
            if !win {
                s1_ind = new_s1_ind;
                s2_ind = new_s2_ind;
            } else {
                break;
            }
            s1_ind +=1;
            s2_ind +=1;
        } else if (s1_ch == ']' && s2_ch == ']') {
            s1_ind +=1;
            s2_ind +=1;
        } else if (s1_ch == ']' && s2_ch.is_digit(10)) {
            // s1 probably was an empty list
            println!("Integers were in the correct orders");
            break;
        } else if (s2_ch == ']' && s1_ch.is_digit(10)) {
            // s2 probably was an empty list
            println!("Integers were in the incorrect order");
            break;
        } 
        //println!("cond 1:{} cond2:{}", s1_ch != ']', s2_ch == ']');
        thread::sleep(time::Duration::from_secs(2));
    }

}

fn solve(packets: Vec<(String, String)>) {
    for (pack_index, packet) in packets.iter().enumerate() {
        println!("{:?}", packet);
        let mut new_packet = solve_mixed_types(packet);
        println!("{:?}", new_packet);
        check_ordering(new_packet.0, new_packet.1, pack_index);
        
    }
}

fn main() {
    let mut packets = read_file("input-test.txt");
    //let mut packet = ("[[9,6], 1]".to_string(), "[[8,7,6], [1]]".to_string());
    //let mut packets = Vec::new();
    //packets.push(packet);
    solve(packets);
}
