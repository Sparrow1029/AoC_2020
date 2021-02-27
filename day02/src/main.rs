use helpers;
use std::{
    path::PathBuf,
    ops::RangeInclusive,
};

fn get_line_data(line: &String) -> ((i32, i32), char, &str) {
    let mut values = line.split(" ");
    let range_str = values.next().unwrap().split("-");
    let ints: Vec<i32> = range_str.map(|s| s.parse().expect("no")).collect();
    (
        (ints[0], ints[1]),  // two integers at front of string (ex: "1-4")
        values.next().unwrap().chars().next().unwrap(),  // get rid of trailing ':' and get as `char` primitive
        values.next().unwrap()  // &str of actual password "abxbbzbxyzzaa"
    )
}

fn part_1(path: &PathBuf) -> i32 {
    let file_data = helpers::lines_from_file(path);
    let mut valid_passwords = 0;
    for line in file_data.iter() {
        let data = get_line_data(line);
        let counts = helpers::char_counter(data.2.to_string());
        let char_count_in_pwd = counts.get(&data.1).unwrap_or(&0i32);
        // password is valid only if character count is in specified range
        if RangeInclusive::new(data.0.0, data.0.1).contains(char_count_in_pwd) {
            valid_passwords += 1;
        }
    }
    valid_passwords
}

fn part_2(path: &PathBuf) -> i32 {
    let file_data = helpers::lines_from_file(path);
    let mut valid_passwords = 0;
    for line in file_data.iter() {
        let (pos, char_, psswd) = get_line_data(line);
        let char_pos_1 = psswd.chars().nth(pos.0 as usize - 1).unwrap();
        let char_pos_2 = psswd.chars().nth(pos.1 as usize - 1).unwrap();
        // cast booleans to 0 || 1 to ensure character appears once, and only once in string
        let char_count = (char_pos_1 == char_) as i32 + (char_pos_2 == char_) as i32;
        if char_count == 1 {
            valid_passwords += 1
        }
    }
    valid_passwords
}

fn main() {
    let input_file_path = PathBuf::from(helpers::DATA_FILE_PATH).join("day02_input.txt");
    println!("PART 1 (number of valid passwords): {}", part_1(&input_file_path));
    println!("PART 2 (number of valid passwords): {}", part_2(&input_file_path));
}
