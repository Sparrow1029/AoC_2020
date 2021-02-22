use::helpers::{get_data_filepath,lines_from_file,};

fn number_of_trees_in_path(tree_map: &Vec<String>, x_slope: u64, y_slope: u64) -> u64 {
    let mut tree_count: u64 = 0;
    let row_len: &usize = &tree_map[0].len();
    let mut cur_col: u64 = x_slope;
    for line in tree_map.iter().skip(y_slope as usize).step_by(y_slope as usize) {
        let index = cur_col as usize % row_len;
        // debugging
        // println!("{}[{}]{}", &line[..index], &line.chars().nth(index).unwrap().to_string(), &line[index+1..]);
        let symbol: char = line.chars().nth(index).unwrap();
        if symbol == '#' {
            tree_count += 1;
        }
        cur_col += x_slope;
    }
    tree_count
}

fn main() {
    let input_file = get_data_filepath().unwrap().join("day03_input.txt");
    let lines = lines_from_file(input_file);
    // part 1
    println!("{:?}", number_of_trees_in_path(&lines, 3u64, 1u64));

    //part 2
    let mut totals: Vec<u64> = Vec::new();
    let tuples: [(u64, u64); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    for tup in tuples.iter() {
        // println!("\n!!!!! RUNNING {:?} !!!!!\n", tup);
        let (x, y) = *tup;
        totals.push(number_of_trees_in_path(&lines, x, y));
    }
    println!("{:?}", totals);
    let product: u64 = totals.into_iter().product();
    println!("{}", product);
}
