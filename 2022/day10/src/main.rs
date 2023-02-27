use std::fs;


fn read_file(filename: &str) {

    let mut cycle_num:Vec<i64> = Vec::new();
    let mut file_string = fs::read_to_string(filename).expect("I cannot open this file");

    let mut number_x = 1;
    let mut newnumber_x = 1;

    let mut sum = 0;

    for (i,line) in file_string.lines().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            cycle_num.push(number_x);
            newnumber_x = number_x + parts[1].parse::<i64>().expect("Couldn't parse");
            cycle_num.push(number_x);
        } else {
            cycle_num.push(number_x);
        }

    }
    let sum = (cycle_num[19] * 20) + (cycle_num[59] * 60) + (cycle_num[99] * 100) + (cycle_num[139] * 140) + (cycle_num[179] * 180) + (cycle_num[218] * 220);
    println!("{:?}", cycle_num);
    println!("{}, {}, {}, {}, {}, {}", cycle_num[19] * 20, cycle_num[59] * 60, cycle_num[99] * 100, cycle_num[139] *140, cycle_num[179] * 180, cycle_num[218] * 220);
    println!("The sum is: {}", sum);
    
}


fn main() {
    read_file("input-test.txt");
}
