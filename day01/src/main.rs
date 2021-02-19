use helpers::lines_from_file;
use std::path::PathBuf;

fn part_1(path: &PathBuf) -> Option<i32> {
    let line_data: Vec<i32> = lines_from_file(path)
        .iter().map(|s| s.parse().expect("Couldn't convert to i32"))
        .collect();
    for i in 1..line_data.len()-1 {
        let cur = line_data[i];
        for j in i+1..line_data.len() {
            if line_data[j] + cur == 2020 {
                return Some(line_data[j] * cur);
            }
        }
    }
    return None;
}

fn part_2(path: &PathBuf) -> Option<i32> {
    let line_data: Vec<i32> = lines_from_file(path)
        .iter().map(|s| s.parse().expect("Couldn't parse line to i32"))
        .collect();
    for i in 1..line_data.len()-2 {
        for j in i+1..line_data.len() {
            for k in i+2..line_data.len() {
                if line_data[i] + line_data[j] + line_data[k] == 2020 {
                    return Some(line_data[i] * line_data[j] * line_data[k]);
                }
            }
        }
    }
    return None;
}

fn main() {
    let path = PathBuf::from("/Users/p2910482/Projects/rust/AoC_2020/data_files/day01_input.txt");
    let product_1 = part_1(&path).unwrap();
    println!("{}", product_1);
    let product_2 = part_2(&path).unwrap();
    println!("{}", product_2);
}