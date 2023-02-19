use std::{fs, collections::HashMap};



struct DirectoryCrawler {
    directory_sizes: HashMap<String, u64>,
    current_directory: String
}

fn change_dir(dir_crawler: &mut DirectoryCrawler, line: &str) {
    let split: Vec<&str> = line.split_whitespace().collect();
    let result = split[2];

    if result.contains(".."){
        let pos = dir_crawler.current_directory.rfind("/").expect("There were no slashes in current directory");
        dir_crawler.current_directory = dir_crawler.current_directory[..pos].to_string();
    } else if result.contains("/") {
        dir_crawler.current_directory.push_str(result);
    } else {
        dir_crawler.current_directory.push_str(&format!("/{}", result));
    }
    println!("{}", line);
    println!("Current directory is: {}", dir_crawler.current_directory);

    // initialize dir score
    dir_crawler.directory_sizes.insert(dir_crawler.current_directory.to_string(), 0);
}


fn add_file(dir_crawler: &mut DirectoryCrawler, line: &str) {
    let split: Vec<&str> = line.split_whitespace().collect();
    let result = split[0].parse::<u64>().expect("Could not parse filesize string.");

    let current_score = dir_crawler.directory_sizes.get(&dir_crawler.current_directory).expect("Could not find directory in directory scores map.");

    dir_crawler.directory_sizes.insert(dir_crawler.current_directory.to_owned(), current_score + result);

}

fn solve(filename: &str) {

    let binding = fs::read_to_string(filename).expect("Could not read file");
    let mut lines = binding.lines();
    let mut dir_crawler = DirectoryCrawler {
        current_directory: "".to_string(),
        directory_sizes: HashMap::new(),
    };

    for line in lines {

        if line.starts_with("$") {
            if line.contains("cd") {
                change_dir(&mut dir_crawler, line)
            } 
        } else {
            add_file(&mut dir_crawler, line);
        }
    }
}



fn main() {
    solve("input-test.txt");
    
}
