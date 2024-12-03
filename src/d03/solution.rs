use std::fs;
use regex::Regex;

pub fn solve() {
    let file_path = "src/d03/input.txt";
    let contents = parse_file(file_path);
    println!("Part 1: {}", file_to_regex_matches(contents.clone()));
    println!("Part 2: {}", file_to_regex_matches(remove_between_flags(contents.clone())));
}

fn parse_file(file_path: &str) -> String {
    match read_file(file_path) {
        Ok(contents) =>
            contents,
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            String::new()
        }
    }
}

fn read_file(file_path: &str) -> std::io::Result<String> {
    let contents: String = fs::read_to_string(file_path)?;
    Ok(contents)
}

fn file_to_regex_matches(file_string: String) -> u32 {
    let re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

    let mut sum = 0;

    for captures in re.captures_iter(&file_string) {
        let first: u32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let second: u32 = captures.get(2).unwrap().as_str().parse().unwrap();
        sum += first * second;
    }
    sum
}

fn remove_between_flags(file_string: String) -> String {
    let re = Regex::new(r"(?s)don't\(\).*?do\(\)").unwrap();
    let result = re.replace_all(&file_string, "").to_string();
    return result;
}