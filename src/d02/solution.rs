use std::fs;

pub fn solve() {
    let file_path = "src/d02/input.txt";
    let lists = parse_file(file_path);

    let safe_list: Vec<Vec<i32>> = lists
    .iter()
    .filter(|list: &&Vec<i32>| can_be_safe(list, false)).
    cloned().
    collect();
    println!("Part 1: {}", safe_list.len());

    
    let can_be_safe_list: Vec<Vec<i32>> = lists
        .iter()
        .filter(|list: &&Vec<i32>| can_be_safe(list, true)).
        cloned().
        collect();
    println!("Part 2 {}", can_be_safe_list.len());
}

fn parse_file(file_path: &str) -> Vec<Vec<i32>> {
    match read_file(file_path) {
        Ok(contents) =>
            file_string_to_list(contents),
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            vec![]
        }
    }
}

fn read_file(file_path: &str) -> std::io::Result<String> {
    let contents: String = fs::read_to_string(file_path)?;
    Ok(contents)
}

fn file_string_to_list(file_string: String) -> Vec<Vec<i32>> {
    let mut lines: Vec<Vec<i32>> = vec![];

    for line in file_string.lines() {
        let integers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap()) 
            .collect();
        lines.push(integers);
    }
    lines
}

fn direction_of_number(num: i32) -> i32 {
    if num > 0 {
        1
    } else if num < 0 {
        -1
    } else {
        0
    }
}

fn can_be_safe(list: &Vec<i32>, is_first: bool) -> bool {
    let mut dir: i32 = 0;
    let mut current_index: usize = 0;
    let first_two = &list[0..2];
    if first_two[0] == first_two[1] {
        return maybe_try_again(is_first, 0, list);
    }

    for pair in list.windows(2) {
        if let [a, b] = pair {
            let diff = a - b;
            let new_dir = direction_of_number(diff);
            if diff.abs() == 0 || diff.abs() > 3 {
                return maybe_try_again(is_first, current_index, list) || maybe_try_again(is_first, current_index + 1, list);
            } else if dir == 0 {
                dir = new_dir;
            } else {
                if dir != new_dir {
                    return maybe_try_again(is_first, current_index - 1, list) || maybe_try_again(is_first, current_index, list) || maybe_try_again(is_first, current_index + 1, list);
                }
            }
        };
        current_index += 1;
    }
    return true;
}

fn maybe_try_again(is_first: bool, current_index: usize, list: &Vec<i32>) -> bool {
    if is_first {
        let new_list: Vec<i32> = list
            .iter()
            .enumerate()
            .filter(|&(index, _)| index != current_index)
            .map(|(_, &value)| value)
            .collect();

        let ret = can_be_safe(&new_list, false);

        return ret;
    } else {
        return false
    }
}