use std::{collections::{HashMap,HashSet}, fs::File, io::{self, Read}, ops::Index};
use helpers::{InputData,get_parsed_data};

const VALID_KEYARR: [&str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

fn get_valid_keys_hashset<'a>() -> HashSet<&'a str> {
    let mut hashset = HashSet::new();
    for &key in VALID_KEYARR.iter() {
        hashset.insert(key);
    }
    hashset
}

fn get_pgraph_blocks_from_file() -> Vec<String> {
    let data: InputData = get_parsed_data("day04_input.txt");
    let blocks = data.as_string.split("\n\n").collect::<Vec<&str>>();
    let mut final_strings = Vec::new();
    for block in blocks {
        &final_strings.push(block.replace("\n", " ").trim().to_string());
    }
    final_strings
}

fn get_arr_of_tups_from_string(string: &String) -> Vec<(&str, &str)> {
    let mut tuples: Vec<(&str, &str)> = Vec::new();
    for pair_str in string.split(" ") {
        let kv = pair_str.split(":").collect::<Vec<&str>>();
        tuples.push((kv[0], kv[1]));
    }
    tuples
}

// fn get_passport_from_block(pgraph: &String) {
// }

fn main() {
    // println!("{:?}", get_pgraph_blocks_from_file())
    for block in get_pgraph_blocks_from_file() {
        println!("{:?}", get_arr_of_tups_from_string(&block))
    }
}