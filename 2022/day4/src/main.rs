use std::fs;


fn read_file(filename: &str) -> Vec<((u32, u32), (u32, u32))> {

    let mut file_string = fs::read_to_string(filename).expect("File could not be read");
    let mut pairs = Vec::new();
    for line in file_string.lines() {
        let ints: Vec<u32> = line.split([',', '-']).map(|s| s.parse().expect("Could not convert")).collect();
        pairs.push(((ints[0], ints[1]), (ints[2], ints[3])));
    }
    return pairs;
}

fn read_file2(filename: &str) -> (Vec<(u32, u32)>, u32) {

    // A mistake was made and this was not the way to solve this problem
    let mut max: u32 = 0;
    let mut file_string = fs::read_to_string(filename).expect("File could not be read");
    let mut pairs = Vec::new();
    for line in file_string.lines() {
        let ints: Vec<u32> = line.split([',', '-']).map(|s| s.parse().expect("Could not convert")).collect();
        for int in &ints {
            if int > &max {
                max = *int;
            }
        }
        pairs.push((ints[0], ints[1]));
        pairs.push((ints[2], ints[3]));
    }
    return (pairs, max);
}


fn solve_pairs(pairs: Vec<(u32, u32)>, max: u32) {

    let mut score_arr = vec![0; max as usize];

    for pair in pairs {

        for i in pair.0..pair.1 {
            score_arr[i as usize] += 1;
        }
    }

    println!("The socre arr is: {:?}", score_arr);
}




fn main() {
    
    let pairs = read_file("input.txt");
    let mut count = 0;
    for pair in pairs {

        if pair.0.0 >= pair.1.0 && pair.0.1 <= pair.1.1 {
            count +=1;
        } else if pair.1.0 >= pair.0.0 && pair.1.1 <= pair.0.1 {
            count +=1;
        }
    }
    println!("count is: {}", count);


    let pairs = read_file("input.txt");
    let mut count2 = 0;

    for pair in pairs {

        if pair.0.0 <= pair.1.0 && pair.0.1 >= pair.1.0  {
            // .234........
            // ...4567.....
            println!("Adds count with: {:?}", pair);
            count2 += 1;
        } else if pair.1.0 <= pair.0.0 && pair.1.1 >= pair.0.0 {
            // ...456......
            // ..345.......
            count2 += 1;
            println!("Adds count with: {:?}", pair);
        }
    }
    println!("The second count is: {}", count2);
}
