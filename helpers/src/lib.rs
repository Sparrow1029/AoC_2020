use std::{
    fs::{File, canonicalize},
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
    collections::HashMap,
};

pub const DATA_FILE_PATH: &str = "/Users/p2910482/Projects/rust/AoC_2020/data_files";

pub fn get_data_filepath() -> Option<PathBuf> {
    let srcdir = PathBuf::from("./../data_files/.");
    let data_file_dir: PathBuf = canonicalize(&srcdir).expect("Bad data_file_path");
    Some(data_file_dir)
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn char_counter(input_string: &str) -> HashMap<char, i32> {
    let letter_counts: HashMap<char, i32> =
        input_string
            .to_lowercase()
            .chars()
            .fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            });
    return letter_counts
}

mod lib_tests {
    #[test]
    fn test_char_count() {
        let counts = super::char_counter("Hello world".to_string());
        // println!("{:?}", counts);
        let l: i32 = *counts.get(&'l').unwrap();  // what the FUCK rust??
        assert_eq!(l, 3i32);
    }

    #[test]
    fn test_lines_from_file() {
        let file_data = super::lines_from_file("/Users/p2910482/Projects/rust/AoC_2020/helpers/src/test_file.txt");
        // println!("{:?}", file_data);
        assert_eq!(file_data[4], "X-Wing");
    }
}