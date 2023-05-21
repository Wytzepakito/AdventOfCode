use queues::*;
use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn read_file(filename: &str) -> (Vec<Vec<char>>, Vec<(i32, i32)>, (i32, i32)) {
    let mut file_string = fs::read_to_string(filename).expect("Could not read file");
    let mut end = (0, 0);
    let mut starts = Vec::new();
    let mut matrix = Vec::new();

    for (y, line) in file_string.lines().enumerate() {
        let mut row = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            
            if ch == 'a' {
                starts.push((x as i32, y as i32));
            }
            if ch == 'E' {
                end = (x as i32, y as i32);
                row.push('z');
            } else if ch == 'S' {
                starts.push((x as i32, y as i32));
                row.push('a');
            } else {
                row.push(ch)
            }
        }
        matrix.push(row);
    }
    return (matrix, starts, end);
}

fn solve(grid: & Vec<Vec<char>>, start: (i32, i32), end: (i32, i32)) -> u32 {
    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut dist_hash: HashMap<(i32, i32), u32> = HashMap::new();

    let x_max = grid[1].len() as i32;
    let y_max = grid.len() as i32;

    let mut d = VecDeque::new();
    d.push_back(start);
    dist_hash.insert(start, 0);

    while (d.len() != 0) {
        let current = d.pop_front().unwrap();
        let current_letter = grid[current.1 as usize][current.0 as usize];
        let current_distance = dist_hash
            .get(&current)
            .expect("Somehow couldn't find current in distance hash")
            .clone();

        for direction in &directions {
            if (current.0 + direction.0 >= 0)
                && (current.0 + direction.0 < x_max)
                && (current.1 + direction.1 >= 0)
                && (current.1 + direction.1 < y_max)
            {
                let new_spot = (current.0 + direction.0, current.1 + direction.1);
                let new_letter = grid[new_spot.1 as usize][new_spot.0 as usize];

                if dist_hash.contains_key(&new_spot) {
                    let distance_to_new = dist_hash
                        .get(&new_spot)
                        .expect("Somehow couldn't find new_spot in distanc hash, should be there");
                    if (current_letter as u32 + 1 >= new_letter as u32)
                    {
                        if distance_to_new > &(current_distance + 1) {
                            // Update disthash with shorter distance
                            dist_hash.insert(new_spot, current_distance + 1);
                        }
                    }
                } else {

                    if (current_letter as u32 + 1 >= new_letter as u32)
                    {
                        dist_hash.insert(new_spot, current_distance + 1);
                        d.push_back(new_spot);
                    }
                }
            }
        }
    }
    if dist_hash.contains_key(&end) {
        println!("one path has length: {}", dist_hash.get(&end).unwrap());
        return dist_hash.get(&end).unwrap().clone();
    } 
    return(u32::MAX)
}

fn main() {
    let (grid, starts, end) = read_file("input.txt");
    let mut shortest_path = u32::MAX;
    for start in starts {
        let path = solve(&grid, start, end);
        if path < shortest_path {
            shortest_path = path;
        }
    }
    println!("The shortest path was: {}", shortest_path);
}
