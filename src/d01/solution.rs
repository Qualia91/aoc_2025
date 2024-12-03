use std::{collections::HashMap, fs};

pub fn solve() {
    let file_path = "src/d01/input.txt";
    let (mut list_a, mut list_b) = parse_file(file_path);

    list_a.sort();
    list_b.sort();

    let size: usize = list_a.len() - 1;

    let mut sum = 0;
    for i in 0..=size{
        sum += (list_a[i] - list_b[i]).abs()
    }
    println!("Part 1: {}", sum);

    // Part 2

    // calc how many of each number in list 2 there are
    let frequency_map = list_b.iter().fold(
        HashMap::new(),
        |mut acc, &item| {
        *acc.entry(item).or_insert(0) += 1;
        acc}
    );

    let mut overall_score: i32 = 0;
    for a in list_a {
        overall_score += frequency_map.get(&a).unwrap_or(&0) * a;
    }
    println!("Part 2: {}", overall_score);
}

fn parse_file(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    match read_file(file_path) {
        Ok(contents) =>
            file_string_to_lists(contents),
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            (vec![], vec![])
        }
    }
}

fn read_file(file_path: &str) -> std::io::Result<String> {
    let contents: String = fs::read_to_string(file_path)?;
    Ok(contents)
}

fn file_string_to_lists(file_string: String) -> (Vec<i32>, Vec<i32>) {
    let mut items_a: Vec<i32> = vec![];
    let mut items_b: Vec<i32> = vec![];

    for line in file_string.lines() {
        let mut parts = line.split_whitespace();
        let num1: i32 = parts.next().unwrap_or("").parse().unwrap();
        let num2: i32 = parts.next().unwrap_or("").parse().unwrap();
        items_a.push(num1);
        items_b.push(num2);
    }
    (items_a, items_b)
}