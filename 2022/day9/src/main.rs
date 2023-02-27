use std::{fs, collections::HashMap};

fn movement_hash() -> HashMap<&'static str, (i32,i32)> {
    let mut hash = HashMap::new();
    hash.insert("R", (0, 1));
    hash.insert("D", (1, 0));
    hash.insert("L", (0, -1));
    hash.insert("U", (-1, 0));
    hash
}
fn update_tail(head: (i32, i32), tail: &mut (i32, i32), direction: &str) {
    let mut hash = movement_hash();


    if (head.0 - tail.0).abs() == 2 && (head.1 - tail.1).abs() == 2 {
        if tail.0 > head.0 {
            tail.0 = tail.0 - 1;
        } else if tail.0 < head.0 {
            tail.0 = tail.0 + 1;
        } 
        if tail.1 > head.1 {
            tail.1 =  tail.1 - 1;
        } else if tail.1 < head.1 {
            tail.1 = tail.1 + 1;
        }

    }
    if (head.0 - tail.0).abs() == 2 {
        // diagonal move
        if (head.1 - tail.1).abs() == 1 {
            tail.1 = head.1;
        }
        if tail.0 > head.0 {
            tail.0 = tail.0 - 1;
        } else if tail.0 < head.0 {
            tail.0 = tail.0 + 1;
        }
        println!("Moved y");
    } else if (head.1 - tail.1).abs() == 2 {
        // diagonal move
        if (head.0 - tail.0).abs() == 1 {
            tail.0 = head.0;
        }
        if tail.1 > head.1 {
            tail.1 =  tail.1 - 1;
        } else if tail.1 < head.1 {
            tail.1 = tail.1 + 1;
        }
        println!("Moved x");
    }

}

fn make_pos(n: i32) -> Vec<(i32, i32)> {
    let mut position_list = Vec::new();
    for i in 0..n {
        position_list.push((1000, 1000));
    }
    position_list
}


fn update_positions(position_list: &mut Vec<(i32, i32)>, direction: &str) {

    for i in 1..position_list.len() {
        update_tail(position_list[i-1], &mut position_list[i], direction);
    }

}


fn read_file(filename: &str, mov_mat: &mut Vec<Vec<bool>>) {
    // as (y, x)
    let mut position_list = make_pos(10);
    let mut hash = movement_hash();
    let mut update_count = 0;

    let mut file_string = fs::read_to_string(filename).expect("I could not read this file");

    for line in file_string.lines() {

        let parts: Vec<&str> = line.split_whitespace().collect();
        let range: usize = parts[1].parse().expect("Couldn't parse the range part");
        println!("{}", line);

        for i in 0..range {
            // update head
            let movements = hash.get(parts[0]).expect("Could not find part key in hash");
            position_list[0].0 = position_list[0].0 + movements.0;
            position_list[0].1 = position_list[0].1 + movements.1;
            // update tail
            update_positions(&mut position_list, parts[0]);
            println!("Position of first part is: {:?}", position_list[0]);
            println!("Position of second part is: {:?}", position_list[1]);
            println!("Position of third part is: {:?}", position_list[2]);
            println!("Position of fourth part is: {:?}", position_list[3]);
            println!("Position of fifth part is: {:?}", position_list[4]);
            println!("Position of sixth part is: {:?}", position_list[5]);
            println!("Position of seventh part is: {:?}", position_list[6]);
            println!("Position of eigth part is: {:?}", position_list[7]);
            println!("Position of nineth part is: {:?}", position_list[8]);
            println!("Position of tenth part is: {:?}", position_list[9]);
            // update move_mat
            if (mov_mat[position_list[9].0 as usize][position_list[9].1 as usize] == false) {
                mov_mat[position_list[9].0 as usize][position_list[9].1 as usize] = true;
                update_count += 1;
            }

        }
    }
    println!("Positions that the tail visited is: {}", update_count);

}

fn create_movement_matrix(n: i32) -> Vec<Vec<bool>> {

    let mut matrix: Vec<Vec<bool>> = Vec::new();

    for i in 0..n {
        let mut row: Vec<bool> = Vec::new();
        for j in 0..n {
            row.push(false);
        }
        matrix.push(row);
    }
    matrix
}


fn main() {
   let mut mov_mat = create_movement_matrix(2000);
   read_file("input.txt", &mut mov_mat);
}
