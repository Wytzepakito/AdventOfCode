use std::fs;

fn read_file(filename: &str) -> Vec<Vec<u8>> {
    let mut file_string = fs::read_to_string(filename).expect("I could not read this file");
    let mut matrix = Vec::new();

    for line in file_string.lines() {
        let mut row: Vec<u8> = Vec::new();
        for ch in line.split("") {
            row.push(ch.parse().expect("Could not parse string to int"));
        }
        matrix.push(row);
    }
    matrix
}

fn solve(matrix: Vec<Vec<u8>>) {
    for y in 0..matrix.len() {
        // solve right

        // solve left
    }

    for x in 0..matrix.len() {
        // solve downwards

        // solve upwards

    }
}

fn main() {
    let matrix = read_file("input-test.txt");


}
