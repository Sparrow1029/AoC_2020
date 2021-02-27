use std::{collections::{HashMap,HashSet}, fs::File, io::{self, Read}, ops::Index};
use helpers::{
    lines_from_file,
    get_data_filepath
};

const valid_keyarr: [&str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

fn get_valid_keys_hashset<'a>() -> HashSet<&'a str> {
    let mut hashset = HashSet::new();
    for &key in valid_keyarr.into_iter() {
        hashset.insert(key);
    }
    hashset
}

fn file_read_to_string() -> io::Result<String> {
    let path = get_data_filepath().unwrap().join("day04_input.txt");
    let mut file = File::open(&path)?;
    let mut new_string = String::new();
    file.read_to_string(& mut new_string)?;
    Ok(new_string)
}
fn get_pgraph_blocks_from_file() -> Vec<String> {
    let lines = file_read_to_string().unwrap();
    let blocks = lines.split("\n\n").collect::<Vec<&str>>();
    let mut final_strings = Vec::new();
    for block in blocks {
        &final_strings.push(block.replace("\n", " ").trim().to_string());
    }
    final_strings
}

// fn get_arr_of_tups_from_string(string: &String) -> Vec<(&str, &str)> {
// }

// fn get_passport_from_block(pgraph: &String) {
// }

fn main() {
    println!("{:?}", get_pgraph_blocks_from_file())
}