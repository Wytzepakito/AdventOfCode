use std::fs;


fn read_file(filename: &str) {

    let mut cycle_num:Vec<i64> = Vec::new();
    let mut file_string = fs::read_to_string(filename).expect("I cannot open this file");

    let mut number_x = 1;
    let mut newnumber_x = 1;

    let mut sum = 0;
    let mut CRTrows: Vec<&str> = Vec::new();

    for (i,line) in file_string.lines().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            if (number_x >= (cycle_num.len() as i64 % 40) -1 && number_x <= (cycle_num.len() as i64 % 40) + 1) {
                CRTrows.push("#");
            } else {
                CRTrows.push(".");
            }
            cycle_num.push(number_x);
            newnumber_x = number_x + parts[1].parse::<i64>().expect("Couldn't parse");
            if (number_x >= (cycle_num.len() as i64 % 40) -1 && number_x <= (cycle_num.len() as i64 % 40) + 1) {
                CRTrows.push("#");
            } else {
                CRTrows.push(".");
            }
            cycle_num.push(number_x);
            number_x = newnumber_x;
        } else {
            if (number_x >= (cycle_num.len() as i64 % 40) -1 && number_x <= (cycle_num.len() as i64 % 40) + 1) {
                CRTrows.push("#");
            } else {
                CRTrows.push(".");
            }
            cycle_num.push(number_x);
        }

    }
    let sum = (cycle_num[19] * 20) + (cycle_num[59] * 60) + (cycle_num[99] * 100) + (cycle_num[139] * 140) + (cycle_num[179] * 180) + (cycle_num[219] * 220);
    println!("{:?}", cycle_num);
    println!("{}, {}, {}, {}, {}, {}", cycle_num[19] * 20, cycle_num[59] * 60, cycle_num[99] * 100, cycle_num[139] *140, cycle_num[179] * 180, cycle_num[219] * 220);
    println!("The sum is: {}", sum);
    println!("Length of CRTrows is: {}", CRTrows.len());
    for i in 0..6 {
        let slice = &CRTrows[i*40..(i+1) *40];
        println!("{}", slice.join(""));
    }
    
}


fn main() {
    read_file("input.txt");
}
