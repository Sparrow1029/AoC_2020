pub mod grid;

use std::fs::read_to_string;

pub fn get_puzzle_input_string(day: u32) -> anyhow::Result<String> {
    let aoc_home = dotenv::var("AOC_HOME")?;
    let filename = format!("{}/puzzle_inputs/day{:02}.txt", aoc_home, day);
    let data = read_to_string(filename)?;
    Ok(data)
}
