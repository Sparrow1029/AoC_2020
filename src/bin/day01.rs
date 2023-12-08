use aoc_2020::get_puzzle_input_string;
use itertools::Itertools;

const DAY_01: u32 = 1;

fn parse_input() -> Vec<u64> {
    get_puzzle_input_string(DAY_01)
        .expect("I/O Error")
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect()
}

fn part1(input: &[u64]) -> Option<u64> {
    let len = input.len();
    for i in 0..len {
        for j in i + 1..len {
            if input[i] + input[j] == 2020 {
                return Some(input[i] * input[j]);
            }
        }
    }
    None
}

fn part2(input: &[u64]) -> u64 {
    input
        .iter()
        .combinations(3)
        .find_map(|combo| {
            let sum = combo.iter().copied().sum();
            match sum {
                2020 => Some(combo.into_iter().product()),
                _ => None,
            }
        })
        .unwrap()
}

fn main() {
    let input = parse_input();
    println!("Part 1: {}", part1(&input).unwrap());
    println!("Part 2: {}", part2(&input))
}
