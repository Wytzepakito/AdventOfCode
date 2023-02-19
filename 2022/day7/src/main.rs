use std::{collections::HashMap, fs, f64::INFINITY, u64::MAX};

struct DirectoryCrawler {
    directory_sizes: HashMap<String, u64>,
    current_directory: String,
}

fn change_dir(dir_crawler: &mut DirectoryCrawler, line: &str) {
    let split: Vec<&str> = line.split_whitespace().collect();
    let result = split[2];

    if result.contains("..") {
        let pos = dir_crawler
            .current_directory
            .rfind("/")
            .expect("There were no slashes in current directory");
        dir_crawler.current_directory = dir_crawler.current_directory[..pos].to_string();
    } else if result.contains("/") {
        dir_crawler.current_directory.push_str(result);
    } else {
        dir_crawler
            .current_directory
            .push_str(&format!("/{}", result));
    }
    println!("{}", line);
    println!("Current directory is: {}", dir_crawler.current_directory);

    // initialize dir score
    if !dir_crawler
        .directory_sizes
        .contains_key(&dir_crawler.current_directory)
    {
        dir_crawler
            .directory_sizes
            .insert(dir_crawler.current_directory.to_string(), 0);
    }
}

fn add_file(dir_crawler: &mut DirectoryCrawler, line: &str) {
    let split: Vec<&str> = line.split_whitespace().collect();
    let result = split[0]
        .parse::<u64>()
        .expect("Could not parse filesize string.");

    let current_score = dir_crawler
        .directory_sizes
        .get(&dir_crawler.current_directory)
        .expect("Could not find directory in directory scores map.");
    dir_crawler.directory_sizes.insert(
        dir_crawler.current_directory.to_owned(),
        current_score + result,
    );

    // Do the rest of directories

    let keys: Vec<String> = dir_crawler
        .directory_sizes
        .keys()
        .map(|s| s.to_owned())
        .collect();
    for key in keys {
        if dir_crawler.current_directory.contains(&key) && key.ne(&dir_crawler.current_directory) {
            let value = dir_crawler.directory_sizes.get(&key).unwrap();
            dir_crawler
                .directory_sizes
                .insert(key.to_string(), value + result);
        }
    }
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
        } else if !line.starts_with("dir") {
            add_file(&mut dir_crawler, line);
        }
    }

    let mut result: u64 = 0;
    for (key, item) in dir_crawler.directory_sizes.iter() {
        if key.matches("/").count() == 2 {
            println!("dir: {}, size: {}", key, item);
        }
        if item < &100_000 {
            result+= item;
        }
    }
    println!("Summing up directories smaller than 100_000 leads to: {}", result);


    println!("Part two finding a directory which leads to 70_000_000 bytes of free space");
    let target_size = 30_000_000 - (70_000_000 -  dir_crawler.directory_sizes.get("/").expect("Could not find root folder"));

    let mut smallest = MAX;

    for (key, item) in dir_crawler.directory_sizes.iter() {

        if item> &target_size {
            println!("Item was larger than target_size {}", item);
        }

        if item < &smallest && item > &target_size {
            println!("Found a smaller dir {} target_size {}",item, target_size);
            smallest = *item;
        }
    }

    println!("Smallest dir to free up the {} of space is: {}", target_size, smallest);
}

fn main() {
    solve("input.txt");
}
