use std::fs;

fn read_file(filename: &str) -> Vec<Vec<u8>> {
    let mut file_string = fs::read_to_string(filename).expect("I could not read this file");
    let mut matrix = Vec::new();

    for line in file_string.lines() {
        let mut row: Vec<u8> = Vec::new();
        for ch in line.chars() {
            row.push(
                ch.to_string()
                    .parse()
                    .expect("Could not parse string to int"),
            );
        }
        println!("{:?}", row);
        matrix.push(row);
    }
    matrix
}

struct DirectionSolution {
    pub top: bool,
    pub right: bool,
    pub down: bool,
    pub left: bool,
}

impl DirectionSolution {
    pub fn new() -> Self {
        DirectionSolution {
            top: false,
            right: false,
            down: false,
            left: false,
        }
    }

    pub fn is_true(&self) -> bool {
        if self.top || self.right || self.down || self.left {
            return true;
        }
        return false;
    }
}

fn create_solution_matrix(size: usize) -> Vec<Vec<DirectionSolution>> {
    let mut sol_mat: Vec<Vec<DirectionSolution>> = Vec::new();

    for _ in 0..size {
        let mut sol_row: Vec<DirectionSolution> = Vec::new();
        for _ in 0..size {
            sol_row.push(DirectionSolution::new());
        }
        sol_mat.push(sol_row);
    }
    sol_mat
}

fn set_borders(
    x: usize,
    y: usize,
    matrix_len: usize,
    sol_mat: &mut Vec<Vec<DirectionSolution>>,
    sol_score: &mut i32,
) {
    if x == 0 {
        sol_mat[y][x].left = true;
        *sol_score += 1;
        return;
    }

    if y == 0 {
        sol_mat[y][x].top = true;
        *sol_score += 1;
        return;
    }
    if x == matrix_len - 1 {
        sol_mat[y][x].right = true;
        *sol_score += 1;
        return;
    }

    if y == matrix_len - 1 {
        sol_mat[y][x].down = true;
        *sol_score += 1;
        return;
    }
}

fn solve_right(
    start_x: usize,
    y: usize,
    matrix: &Vec<Vec<u8>>,
    sol_mat: &mut Vec<Vec<DirectionSolution>>,
    sol_score: &mut i32,
) -> usize {
    let mut new_x = start_x + 1;

    while new_x != matrix.len() -1 {
        if matrix[y][new_x] >= matrix[y][start_x] {
            // A tree to the right is too big for
            return new_x - start_x;
        }
        new_x += 1;
    }
    if matrix[y][start_x] > matrix[y][new_x] {
        println!(
            "Could make a path right at {}, y: {}, x: {}",
            matrix[y][start_x], y, start_x
        );
        sol_mat[y][start_x].right = true;
        *sol_score += 1;
    }
    return new_x - start_x;
}

fn solve_down(
    x: usize,
    start_y: usize,
    matrix: &Vec<Vec<u8>>,
    sol_mat: &mut Vec<Vec<DirectionSolution>>,
    sol_score: &mut i32,
) -> usize {
    let mut new_y = start_y + 1;

    while new_y != matrix.len() -1 {
        if matrix[new_y][x] >= matrix[start_y][x] {
            // A tree to the bottom is too big
            return new_y - start_y;
        }
        new_y += 1;
    }
    if matrix[start_y][x] > matrix[new_y][x] {
        println!(
            "Could make a path downward at {}, y: {}, x: {}",
            matrix[start_y][x], start_y, x
        );
        sol_mat[start_y][x].down = true;
        *sol_score += 1;
    }
    return new_y - start_y;
}

fn solve_left(
    start_x: usize,
    y: usize,
    matrix: &Vec<Vec<u8>>,
    sol_mat: &mut Vec<Vec<DirectionSolution>>,
    sol_score: &mut i32,
) -> usize {
    let mut new_x = start_x - 1;

    while new_x != 0 {
        if matrix[y][new_x] >= matrix[y][start_x] {
            // A tree to the left is too big
            return start_x - new_x;
        }
        new_x -= 1;
    }
    if matrix[y][start_x] > matrix[y][new_x] {
        println!(
            "Could make a path left at {}, y: {}, x: {}",
            matrix[y][start_x], y, start_x
        );
        sol_mat[y][start_x].left = true;
        *sol_score += 1;
    }
    return start_x - new_x;
}
fn solve_top(
    x: usize,
    start_y: usize,
    matrix: &Vec<Vec<u8>>,
    sol_mat: &mut Vec<Vec<DirectionSolution>>,
    sol_score: &mut i32,
) -> usize {
    let mut new_y = start_y - 1;

    while new_y != 0 {
        if matrix[new_y][x] >= matrix[start_y][x] {
            // A tree to the bottom is too big
            return start_y - new_y;
        }
        new_y -= 1;
    }
    if matrix[start_y][x] > matrix[new_y][x] {
        println!(
            "Could make a path upward at {}, y: {}, x: {}",
            matrix[start_y][x], start_y, x
        );
        sol_mat[start_y][x].top = true;
        *sol_score += 1;
    }
    return(start_y - new_y);
}

fn solve(matrix: Vec<Vec<u8>>, mut sol_mat: Vec<Vec<DirectionSolution>>) {
    let mut sol_score = 0;
    let mut highest_score =1;

    for y in 0..matrix.len() {
        for x in 0..matrix.len() {
            if !sol_mat[y][x].is_true() {
                let mut tree_score = 1;
                if x != matrix.len() -1 {
                // solve right
                let score_right = solve_right(x, y, &matrix, &mut sol_mat, &mut sol_score);
                println!("score_right is {}", score_right);
                tree_score = tree_score * score_right;
                }
                if y != matrix.len() - 1 { 
                // solve down
                let score_down = solve_down(x, y, &matrix, &mut sol_mat, &mut sol_score);
                println!("score_down is {}", score_down);
                tree_score = tree_score * score_down;
                }
                if x != 0 {
                // solve left
                let score_left = solve_left(x, y, &matrix, &mut sol_mat, &mut sol_score);
                println!("score left is {}", score_left);
                tree_score = tree_score * score_left;
                }
                if y != 0 {
                // solve top
                let score_top = solve_top(x, y, &matrix, &mut sol_mat, &mut sol_score);
                println!("score top is {}", score_top);
                tree_score = tree_score * score_top;
                }
                if y == 0 || x == 0 || x == matrix.len() - 1 || y == matrix.len() - 1 {
                    tree_score = 0;
                }
                println!("tree score was {}", tree_score);
                if tree_score > highest_score {
                    highest_score = tree_score;
                }
            }
        }
        println!("solution score is: {}", sol_score);
        println!("Highest scenic socre is: {}", highest_score);
    }
}

fn main() {
    let matrix = read_file("input.txt");
    let solution_matrix = create_solution_matrix(matrix.len());

    solve(matrix, solution_matrix);
}
