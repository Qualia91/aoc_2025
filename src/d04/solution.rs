use std::{collections::HashMap, fs};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn add(&self, other: &Vec2) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    fn scale(&self, factor: i32) -> Self {
        Self::new(self.x * factor, self.y * factor)
    }

    fn dir(&self, other: &Vec2) -> Self {
        Self::new(other.x - self.x, other.y - self.y)
    }
}

pub fn solve() {
    let file_path = "src/d04/input.txt";
    let matrix: Vec<Vec<char>> = parse_file(file_path);

    part_1(matrix.clone());
    part_2(matrix.clone());
}

fn part_1(matrix: Vec<Vec<char>>) {

    let mut start_maps: HashMap<Vec2, Vec<Vec2>> = HashMap::new();
    for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == 'X' {
                let ms = find_surrounding_ms(&matrix, i, j);
                start_maps.insert(Vec2::new(i as i32, j as i32), ms);
            }
        }
    }
    // Remove those with no ms
    let filtered: HashMap<_, _> = start_maps.iter().filter(|(_, value)| !value.is_empty()).collect();


    // find words
    let mut word_sum = 0;
    for (xpos, value) in &filtered {
        for mpos in value.iter() {
            // check for work in that direction
            if has_letter(&matrix, xpos, mpos, 2, 'A') && has_letter(&matrix, xpos, mpos, 3, 'S') {
                word_sum += 1;
            }
        }
    }

    println!("Part 1: {:?}", word_sum);
}

fn part_2(matrix: Vec<Vec<char>>) {

    let mut word_sum = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == 'A' {
                if has_surrounding_x_pattern(&matrix, &Vec2::new(i as i32, j as i32)) {
                    word_sum += 1;
                }
            }
        }
    }
    println!("Part 2: {:?}", word_sum);
}

fn parse_file(file_path: &str) -> Vec<Vec<char>> {
    match read_file(file_path) {
        Ok(contents) =>
            convert_to_matrix(contents),
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

fn convert_to_matrix(contents: String) -> Vec<Vec<char>> {
    let mut matrix = vec![];
    for line in contents.lines() {
        let chars: Vec<char> = line.chars().collect();
        matrix.push(chars);
    }
    return matrix;
}

fn find_surrounding_ms(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<Vec2> {
    let mut ms = vec![];
    for di in -1..=1 {
        for dj in -1..=1 {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            if is_within_matrix(matrix, &Vec2::new(ni as i32, nj as i32)) {
                if matrix[ni as usize][nj as usize] == 'M' {
                    ms.push(Vec2::new(ni, nj));
                }
            }
        }
    }
    return ms;
}

fn has_letter(matrix: &Vec<Vec<char>>, xpos: &Vec2, mpos: &Vec2, length: i32, char: char) -> bool {
    // find direction of work
    let dir = xpos.dir(mpos);
    let letter_pos = dir.scale(length).add(&xpos);
    if is_within_matrix(matrix, &letter_pos) {
        if matrix[letter_pos.x as usize][letter_pos.y as usize] == char {
            return true;
        }
    }
    return false;
}

fn has_surrounding_x_pattern(matrix: &Vec<Vec<char>>, apos: &Vec2) -> bool {
    let top_left_vec = apos.add(&Vec2::new(-1, -1));
    let bottom_right_vec = apos.add(&Vec2::new(1, 1));
    let top_right_vec = apos.add(&Vec2::new(-1, 1));
    let bottom_left_vec = apos.add(&Vec2::new(1, -1));
    if is_within_matrix(matrix, &top_left_vec)
        && is_within_matrix(matrix, &bottom_right_vec)
        && is_within_matrix(matrix, &top_right_vec)
        && is_within_matrix(matrix, &bottom_left_vec) {
        let top_left_letter = matrix[top_left_vec.x as usize][top_left_vec.y as usize];
        let bottom_right_letter = matrix[bottom_right_vec.x as usize][bottom_right_vec.y as usize];
        let top_right_letter = matrix[top_right_vec.x as usize][top_right_vec.y as usize];
        let bottom_left_letter = matrix[bottom_left_vec.x as usize][bottom_left_vec.y as usize];

        return ((top_left_letter == 'M' && bottom_right_letter == 'S') || (top_left_letter == 'S' && bottom_right_letter == 'M'))
        && ((top_right_letter == 'M' && bottom_left_letter == 'S') || (top_right_letter == 'S' && bottom_left_letter == 'M'));
    }
    return false;
}

fn is_within_matrix(matrix: &Vec<Vec<char>>, pos: &Vec2) -> bool {
    return pos.x >= 0 && pos.y >= 0 && pos.x < matrix.len() as i32 && pos.y < matrix[0].len() as i32;
}