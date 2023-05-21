use std::fs;

#[derive(PartialEq)]
enum Cell {
    Empty,
    Prefilled,
    Filled
}

fn read_file(filename: &str) -> Vec<Vec<Cell>> {
    let mut board = Vec::new();
    for i in 0..1000 {
        let mut row = Vec::new();
        for j in 0..1000 {
            row.push(Cell::Empty);
        }
        board.push(row);
    }

    let mut file_string = fs::read_to_string(filename).expect("Cannot find the file");

    let mut highest_y = 0;
    // Set points on board
    for line in file_string.lines() {
        let mut coordinates: Vec<&str> = line.split("->").collect();
        let mut opt_prev_coordinate: Option<(i32, i32)> = None;
        for coordinate in coordinates {
            println!("Current coordinate: {}", coordinate);
            let mut math_val: Vec<i32> = coordinate
                .split(",")
                .map(|s| s.trim().parse().unwrap())
                .collect();
            // In the form (x, y)
            let current_coordinate = (math_val[0], math_val[1]);
            if current_coordinate.1 > highest_y {
                highest_y = current_coordinate.1;
            }
            if let Some(prev_coordinate) = opt_prev_coordinate {
                if prev_coordinate.0 == current_coordinate.0 {
                    let (mut low, mut high) = (0, 0);
                    if prev_coordinate.1 > current_coordinate.1 {
                        (low, high) = (current_coordinate.1, prev_coordinate.1 );
                    } else {
                        (low, high) = (prev_coordinate.1, current_coordinate.1 );
                    }
                    for y in low..=high {
                        board[y as usize][prev_coordinate.0 as usize] = Cell::Prefilled;
                    }
                } else if prev_coordinate.1 == current_coordinate.1 {
                    let (mut low, mut high) = (0, 0);
                    if prev_coordinate.0 > current_coordinate.0 {
                        (low, high) = (current_coordinate.0, prev_coordinate.0 );
                    } else {
                        (low, high) = (prev_coordinate.0, current_coordinate.0 );
                    }
                    for x in low..=high {
                        board[prev_coordinate.1 as usize][x as usize] = Cell::Prefilled;
                    }
                }
            }
            opt_prev_coordinate = Some(current_coordinate)
        }
    }


    // set groundfloor on boad
    for x in 0..1000 {
        board[(highest_y + 2) as usize][x] = Cell::Prefilled;
    }

    return board;
}



fn print_board(board: &Vec<Vec<Cell>>) {
    for (y, row) in board.iter().enumerate() {
        if y >= 0 && y < 13 {
            let mut pretty_arr = Vec::new();
            for (x, cell) in row.iter().enumerate() {
                if x >= 480 && x < 520 {
                    if *cell == Cell::Prefilled {
                        pretty_arr.push("#");
                    } else if *cell == Cell::Filled {
                        pretty_arr.push("+");
                    } else {
                        pretty_arr.push(".");
                    }
                }
            }

            println!("{}", pretty_arr.join(" "));
        }
    }

}



fn has_next(point: &(i32, i32), board: &Vec<Vec<Cell>>) -> bool {

    let (x, y): (usize, usize) = (point.0 as usize, point.1 as usize);
    if board[y + 1][x - 1] == Cell::Empty || board[y + 1][x] == Cell::Empty || board[y+1][x + 1] == Cell::Empty {
        return true;
    } 
    return false;
}


fn make_move(point: &(i32, i32), board: &mut Vec<Vec<Cell>>) -> (i32,i32) {


    let (x, y): (usize, usize) = (point.0 as usize, point.1 as usize);
    board[y][x] = Cell::Empty;
    if board[y + 1][x] == Cell::Empty {
        board[y + 1][x] = Cell::Filled;
        return (x as i32, y as i32 +1);
    } else if board[y + 1][x - 1] == Cell::Empty {
        board[y + 1][x - 1] = Cell::Filled;
        return (x as i32 - 1, y as i32 +1);
    } else if board[y + 1][x + 1] == Cell::Empty {
        board[y + 1][x + 1] = Cell::Filled;
        return (x as i32 + 1, y as i32 + 1);
    } else {
        unreachable!();
    }

    
}

fn check_abyss(point: &(i32, i32)) -> bool {

    
    let (x, y): (usize, usize) = (point.0 as usize, point.1 as usize);
    if (y + 1) == 1000 {
        return true;
    }


    false
}


fn solve() {
    let mut board = read_file("input.txt");
    // (x, y)
    let mut new = (500,0);
    let mut came_to_rest = 0;

    'new_points: while (has_next(&new, &board)) {


        let mut current = new;

        'moves: while(has_next(&current, &board)){

            current = make_move(&current, &mut board);
            // print_board(&board);
            // println!("===============================================================================");

            if (check_abyss(&current)) {
                break 'new_points;
            }

        }
    //print_board(&board);
    came_to_rest += 1;
    new = (500,0);
    }

    println!("There were {} points which came to rest", came_to_rest);
    
    



}


struct Person {
    name:Option<String>,
    age: i32
}






fn main() {
    // tryouts();
    solve();





}

fn tryouts() {

    let mut person = Person{
        name: Some("Alfred".to_string()),
        age: 34
    };

    let name = person.name.take();

    let mut veccie = Vec::new();
    for i in 101..106 {
        veccie.push(i);
    }


    let second = veccie.swap_remove(1);
    println!("{:?}", veccie);

}