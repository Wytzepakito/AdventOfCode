use std::{collections::HashMap, fs};

use regex::Regex;

#[derive(PartialEq)]
enum Cell {
    Beacon,
    Sensor,
    NoBeacon,
    Unknown,
}

fn main() {
    println!("Hello, world!");

    let num: i32 = "-1".parse().ok().unwrap();
    solve2();
}
fn solve2() {
    let mut sensormap = read_file("input.txt");
    solve_for_mat((0,4_000_000), (0,4_000_000), sensormap);
}

fn solve_for_mat(
    x_range: (usize, usize),
    y_range: (usize, usize),
    sensormap: HashMap<(i32, i32), (i32, i32)>,
) {
    let mut candidates = Vec::new();
    for y in y_range.0..y_range.1 {
        for x in x_range.0..x_range.1 {

            let point = (x,y);
            if could_have_beacon(point, &sensormap) {
                candidates.push(point);

            }


        }
    }

    println!("The candidates for where a beacon could be are {:?}", candidates);
}

fn could_have_beacon(point: (usize, usize), sensormap: &HashMap<(i32, i32), (i32, i32)>) -> bool {
    for (sensor, beacon) in sensormap.into_iter() {
        // (x,y)
        let distance = (sensor.0 as i32 - beacon.0 as i32).abs()
            + (sensor.1 as i32 - beacon.1 as i32).abs()
            + 1;

        let current_distance =
            (sensor.0 as i32 - point.0 as i32).abs() + (sensor.1 as i32 - point.1 as i32).abs() + 1;

        if current_distance <= distance {
            return false;
        }

    }
    return true;
}

fn solve() {
    println!("Starting to solve this");
    let mut board = [false; 8_000_000];
    let mut sensormap = read_file("input.txt");
    println!("Read file");
    //print_board(&board);
    //tag_no_beacon_cells(&mut board, sensormap);
    println!("Tagged no beacon cells");
    println!("=====================================================");
    //print_board(&board);
    //print_no_beacons_at(&board)
}

fn read_file(filename: &str) -> HashMap<(i32, i32), (i32, i32)> {
    println!("Created board");

    let mut sensor_map = HashMap::new();

    let mut file_string = fs::read_to_string(filename).expect("Cannot find the file.");

    for line in file_string.lines() {
        let regex = Regex::new(r"-?\d+").unwrap();
        let numbers: Vec<i32> = regex
            .find_iter(&line)
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect();

        // input as (x, y)
        sensor_map.insert(
            (numbers[0], numbers[1]),
            (numbers[2], numbers[3]),
        );
    }

    sensor_map
}

fn print_board(board: &Vec<Vec<Cell>>) {
    for (y, row) in board.iter().enumerate() {
        if y >= 100 && y < 123 {
            let mut pretty_arr = Vec::new();
            pretty_arr.push(format!("{}--", y - 100));
            for (x, cell) in row.iter().enumerate() {
                if x >= 90 && x < 126 {
                    if *cell == Cell::Beacon {
                        pretty_arr.push("B".to_string());
                    } else if *cell == Cell::Sensor {
                        pretty_arr.push("S".to_string());
                    } else if *cell == Cell::NoBeacon {
                        pretty_arr.push("#".to_string());
                    } else if *cell == Cell::Unknown {
                        pretty_arr.push(".".to_string());
                    }
                }
            }

            println!("{}", pretty_arr.join(""));
        }
    }
}

fn tag_no_beacon_cells(
    board: &mut [bool; 8_000_000],
    sensormap: HashMap<(usize, usize), (usize, usize)>,
) {
    for (sensor, beacon) in sensormap.into_iter() {
        // (x,y)
        let distance = (sensor.0 as i32 - beacon.0 as i32).abs()
            + (sensor.1 as i32 - beacon.1 as i32).abs()
            + 1;
        tag_no_beacon_for_sensor(board, sensor, distance);
        if sensor.1 == 3_000_000 {
            board[sensor.0] = false;
        }
        if beacon.1 == 3_000_000 {
            board[beacon.0] = false;
        }
    }
}

fn tag_no_beacon_for_sensor(board: &mut [bool; 8_000_000], sensor: (usize, usize), distance: i32) {
    for (x, y) in (0..distance).zip((0..distance).rev()) {
        let top_left = turn_negs_zero((sensor.0 as i32 - x, sensor.1 as i32 - y));
        let top_right = turn_negs_zero((sensor.0 as i32 + x, sensor.1 as i32 - y));
        let bottom_left = turn_negs_zero((sensor.0 as i32 - x, sensor.1 as i32 + y));
        let bottom_right = turn_negs_zero((sensor.0 as i32 + x, sensor.1 as i32 + y));

        if top_left.1 == 3_000_000 {
            board[top_left.0] = true;
            board[top_right.0] = true;

            // top left to top right
            for i in top_left.0..top_right.0 {
                board[i] = true;
            }
        }

        if bottom_left.1 == 3_000_000 {
            board[bottom_left.0] = true;
            board[bottom_right.0] = true;

            // bottom left to bottom right
            for i in bottom_left.0..bottom_right.0 {
                board[i] = true;
            }
        }
    }
}

fn turn_negs_zero(cell: (i32, i32)) -> (usize, usize) {
    let mut result: (usize, usize) = (0 as usize, 0 as usize);
    if cell.0 > 0 {
        result.0 = cell.0 as usize;
    }
    if cell.1 > 0 {
        result.1 = cell.1 as usize;
    }
    result
}

fn print_no_beacons_at(board: &[bool; 8_000_000]) {
    let mut count = 0;

    for cell in board {
        if *cell == true {
            count += 1;
        }
    }

    println!("Row contained {} NoBeacon cells.", count);
}
